#![allow(unused)]


use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use futures::TryFutureExt;
use jsonwebtoken::{Algorithm, decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::ErrorKind;
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::EncodePublicKey;
use rsa::RsaPublicKey;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Thing, thing};
use surrealdb::Surreal;
pub use tracing::{debug, error};
use tracing::info;

use Error::EmailExists;

use crate::{AppState, CFG};
use crate::error::Error;
use crate::Error::TokenExpired;
use crate::model::{
	auth::{AuthData, Claims}, 
	AgentRecord, 
	OperatorRecord
};
// Todo REWORK AGENT AUTHENTICATION // Think before you do
// Plan stuff, design the system before starting it
// Consider a TOTP rather than a token


// Todo rewrite doc
/// JWT Authentication middleware
///
/// Reads a JWT token from the `Authorization` Header \
/// Check its Claims to verify the use can access this page \
/// Returns an error and blocks the request if the token is invalid,  //InvalidSignature //InvalidKeyFormat
pub async fn auth_middleware(
	State(state): State<Arc<AppState>>,
	mut req: Request, next: Next,
) -> Result<Response, Error> {
	// 
	state.db.signin(Root {
		username: &CFG.db.user,
		password: &CFG.db.pass,
	}).await?;
	state.db.use_ns(&CFG.db.ns)
		.use_db(&CFG.db.db)
		.await?;

	// Extract the token from the Authorization header
	let auth_header = match req.headers_mut().get(http::header::AUTHORIZATION) {
		Some(val) => val.to_str().map_err(|_| Error::WrongCredentials)?,
		None => return Err(Error::PermissionDenied),
	};
	let mut header = auth_header.split_whitespace();
	let (_, token) = (header.next(), header.next().ok_or(Error::PermissionDenied)?);


	let headers = jsonwebtoken::decode_header(token).map_err(|e| Error::PermissionDenied)?;
	let key_id = headers.kid.ok_or(Error::PermissionDenied)?;
	let private_key = state.keys.get(key_id.as_str()).ok_or(Error::InternalError)?;
	let decoding_key = DecodingKey::from_rsa_pem(
		RsaPublicKey::from(private_key)
			.to_public_key_pem(LineEnding::LF)
			.map_err(|_| Error::InternalError)?
			.as_bytes()
	).map_err(|_| Error::InternalError)?;
	let token_data = decode_token(token, &decoding_key).await.map_err(|_| { 
		Error::InternalError
	})?;

	let thing = Thing::from_str(&token_data.id.clone().unwrap()).map_err(|_| Error::InternalError)?;
	req.extensions_mut().insert(AuthData {
					jwt: token_data.clone(),
					rec: get_operator_record(&thing, &state.db).await?,
				});
	Ok(next.run(req).await)
}


async fn decode_token(token: &str, key: &DecodingKey) -> Result<Claims, Error> {
	let mut validation = Validation::new(Algorithm::RS256);
	validation.set_audience(&["Mycelium"]);
	validation.set_required_spec_claims(&["exp", "sub", "aud", "nbf", "iss"]);
	validation.algorithms = vec![Algorithm::RS256];
	match decode::<Claims>(token, key, &validation) {
		Ok(data) => Ok(data.claims),
		Err(err) => Err(match err.kind() {
			ErrorKind::ExpiredSignature => Error::TokenExpired,
			_ => {
				error!(err=err.to_string(), token, "Failed to decode token.");
				Error::PermissionDenied
			}
		}),
	}
}


pub async fn get_operator_record(operator_id: &Thing, db: &Surreal<Any>) -> Result<OperatorRecord, Error> {
	let response: Option<OperatorRecord> = db.select(operator_id).await?;
	response.ok_or_else(|| {
		error!(record=operator_id.to_string(), "Can't find the record in the database."); 
		Error::InternalError
	})
}
