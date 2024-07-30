use std::sync::Arc;
use axum::extract::State;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::Utc;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::EncodePrivateKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{AppState, CFG};



#[non_exhaustive]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Claims {
	/// Issued At Time - Time at which the JWT was issued.
	pub iat: Option<i64>,
	/// Not Before Time - Time before which the JWT must not be accepted for processing.
	pub nbf: Option<i64>,
	/// Expiration Time - Time after which the JWT expires.
	pub exp: Option<i64>,
	/// Issuer - Identifies the principal that issued the JWT.
	pub iss: Option<String>,
	/// JWT ID - Unique identifier for this JWT
	pub jti: Option<String>,
	/// Namespace - The SurrealDB Namespace the token is intended for.
	pub ns: Option<String>,
	/// Database - The Database Namespace the token is intended for.
	pub db: Option<String>,
	/// The Access method this token is intended for.
	pub ac: Option<String>,
	/// The identifier of the record associated with the token.
	pub id: Option<String>,
	/// SurrealDB System user roles (like `Owner` or `Editor`)
	pub rl: Option<Vec<String>>,
}
impl Claims {
	pub fn new(id: String, ac: String) -> Self {
		Claims {
			iat: Some(Utc::now().timestamp()),
			nbf: Some(Utc::now().timestamp()),
			exp: Some((Utc::now() + CFG.jwt.ttl).timestamp()),
			jti: Some(Uuid::now_v7().to_string()),
			id:  Some(id),
			iss: Some(CFG.jwt.iss.to_string()),
			ns:  Some(CFG.db.ns.to_string()),
			db:  Some(CFG.db.db.to_string()),
			ac:  Some(ac),
			rl:  None,
		}
	}
}
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/token", get(token_handler))
		.route("/jwks", get(jwks_handler))
		.with_state(app_state)
}

pub async fn jwks_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
	Json(&state.jwks).into_response()
}

pub async fn token_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
	let mut header = Header::new(Algorithm::RS256);
	header.kid = Some("key1".to_string());
	
	// FIXME => Temporary hardcoded values
	let claims = Claims::new("operator:john".to_string(), "operator".to_string());

	let private_key = state.keys
		.get("key1")
		.unwrap()// TODO Clean all those unwraps 
		.to_pkcs8_pem(LineEnding::LF)
		.unwrap(); // TODO: do better
	
	let enc_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
	let token = encode(&header, &claims, &enc_key).unwrap();

	Json(serde_json::json!({
        "token": token,
    }))
}