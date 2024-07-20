use std::sync::Arc;
use axum::extract::{State, Request};
use axum::http;
use axum::response::Response;
use axum::middleware::Next;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::config::Ttl;
use crate::error::Error;
use crate::model::{Claims, TokenType};
use crate::routes::operator::query_operator_by_id;


/// Structure containing the authentication data that will be sent after a successful login
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
	access_token: String,
	token_type: String,
}
impl AuthBody {
	pub fn new(access_token: String) -> Self {
		Self {
			access_token,
			token_type: "Bearer".to_string(),
		}
	}
}

/// Uses a secure RNG to generate a public/private key pair for JWT encoding
pub fn generate_encryption_keys() -> (EncodingKey, DecodingKey) {
	let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
	let encoding_key = EncodingKey::from_ed_der(doc.as_ref());
	let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
	let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());
	(encoding_key, decoding_key)
}




/// Generates a new JWT token
///
/// !!  DOES NOT check for credentials \
///     it just generates whatever token you asked for
///
/// Returns a AuthError::TokenCreation if we failed to create a token
pub fn generate_token(typ: TokenType, id: &Uuid, encoding_key: &EncodingKey, ttl: Ttl) -> Result<String, Error> {
	let now = Utc::now();
	let ttl: chrono::TimeDelta = match typ {
		TokenType::Agent => ttl.agents,
		TokenType::Operator => ttl.operators,
	};
	let claim = Claims {
		sub: *id,
		iat: now.timestamp() as usize,
		exp: (now + ttl).timestamp() as usize,
		typ
	};
	
	encode(
		&Header::new(Algorithm::EdDSA),
		&claim,
		encoding_key
	).map_err(|_| Error::TokenCreation)
}

/// Validates a JWT token
///
/// returns the JWT payload if the token is valid
/// and returns a `AuthError::InvalidToken` if the token is invalid
pub fn validate_token(token: &str, decoding_key: &DecodingKey) -> Result<Claims, Error> {
	let mut validation = Validation::new(Algorithm::EdDSA);
	validation.set_required_spec_claims(&["exp", "sub", "iat"]);
	validation.algorithms = vec![Algorithm::EdDSA];

	match decode::<Claims>(token, decoding_key, &validation) {
		Ok(data) => Ok(data.claims),
		Err(err) => {
			Err( match err.kind() {
				ErrorKind::ExpiredSignature => Error::TokenExpired,
				_ => {
					tracing::error!("Failed to validate token : {err}");
					Error::TokenInvalid
				}
			})
		}
	}
}

/// JWT Authentication middleware
/// 
/// Reads the `Authorization` HTTP Header to recover the token \
/// Then validates the token and search for its owner in the database \
/// If the owner is found and the token is valid, it stores the `Operator` struct in the request's extensions
pub async fn auth(
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response, Error> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header = match auth_header {
		Some(val) => val.to_str().map_err(|_| Error::PermissionDenied)?,
		None => return Err(Error::PermissionDenied),
	};

	let mut header = auth_header.split_whitespace();
	let (_bearer, token) = (header.next(), header.next().ok_or(Error::PermissionDenied)?);
	let claims = validate_token(token, &state.keys.decoding_key)?;

	let operator = query_operator_by_id(&claims.sub, &state.db).await
		.map_err(|_| Error::TokenInvalid)?;

	req.extensions_mut().insert(operator);
	Ok(next.run(req).await)
}
