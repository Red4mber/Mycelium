use std::sync::Arc;
use std::time::SystemTime;
use axum::extract::State;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::Utc;
use jsonwebtoken::{Algorithm, Header};
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
	let token = "";
	let mut header = Header::new(Algorithm::RS256);
	header.kid = Some("key1".to_string());
	let claims = Claims {
		iat: Some(Utc::now().timestamp()),
		nbf: Some(Utc::now().timestamp()),
		exp: Some((Utc::now() + &CFG.jwt.ttl).timestamp()),
		jti: Some(Uuid::now_v7().to_string()),
		iss: Some(&CFG.jwt.iss),
		id:  Some("operator:john".to_string()),
		ns:  Some(&CFG.db.ns),
		db:  Some(&CFG.db.db),
		ac:  Some("operator".to_string()),
		rl:  None,
	};

	Json(serde_json::json!({
        "token": token,
    }))

}