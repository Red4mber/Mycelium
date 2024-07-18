use std::sync::Arc;
use axum::extract::{State, Request};
use axum::http;
use axum::http::StatusCode;
use axum::response::Response;
use axum::middleware::Next;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::error::{AuthError, AuthErrorType};
use crate::model::Claims;
use crate::routes::operator::query_operator_by_id;


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


/// Generates a new JWT token for the operator
///
/// !! DOES NOT check for credentials or token validity \
/// It simply does whatever you ask it to
///
/// Returns a AuthError::TokenCreation if we failed to create a token
pub fn generate_token(id: &Uuid, encoding_key: &EncodingKey) -> Result<String, AuthErrorType> {
	let now = Utc::now();
	let expire: chrono::TimeDelta = Duration::hours(24);

	let claim = Claims {
		sub: *id,
		iat: now.timestamp() as usize,
		exp: (now + expire).timestamp() as usize
	};
	let token = encode(
		&Header::new(Algorithm::EdDSA),
		&claim,
		encoding_key
	);
	token.map_err(|err| {
		tracing::error!("Failed to generate token : {err}");
		AuthErrorType::TokenCreation
	})
}

/// Validates a JWT token
///
/// returns the JWT payload if the token is valid
/// and returns a `AuthError::InvalidToken` if the token is invalid
pub fn validate_token(token: &String, decoding_key: &DecodingKey) -> Result<Claims, AuthErrorType> {
	let mut validation = Validation::new(Algorithm::EdDSA);
	validation.set_required_spec_claims(&["exp", "sub", "iat"]);
	validation.algorithms = vec![Algorithm::EdDSA];

	match decode::<Claims>(&token, &decoding_key, &validation) {
		Ok(data) => {
			Ok(data.claims)
		}
		Err(err) => {
			tracing::error!("Failed to validate token : {err}");
			Err(AuthErrorType::InvalidToken)
		}
	}
}

/// JWT Authentication middleware
/// 
/// Decodes the current user's token to extract the claims then, 
/// checks if everything is valid and matches the user info.
/// 
pub async fn auth(
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response, AuthError> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header = match auth_header {
		Some(val) => val.to_str().map_err(|_| AuthError {
			message: "Failed to read token from the HTTP headers".to_string(),
			status_code: StatusCode::FORBIDDEN
		})?,
		None => return Err(AuthError {
			message: "Access Denied - Please log in".to_string(),
			status_code: StatusCode::FORBIDDEN
		}),
	};
	let mut header = auth_header.split_whitespace();
	let (_bearer, token) = (header.next(), header.next());
	let operator_id = match validate_token(&token.unwrap().to_string(), &state.decoding_key) {
		Ok(claims) => { Ok(claims.sub) }
		Err(_) => return Err(AuthError {
			message: "Unable to decode token".to_string(),
			status_code: StatusCode::UNAUTHORIZED
		})
	}?;
	let current_operator = query_operator_by_id(&operator_id, &state.db).await.map_err(
		|_e| AuthError {
			message: "You are not authorized to consult this resource".to_string(),
			status_code: StatusCode::UNAUTHORIZED
		}
	);
	req.extensions_mut().insert(current_operator);
	Ok(next.run(req).await)
}
