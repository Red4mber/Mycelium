use std::sync::Arc;
use axum::body::Bytes;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::auth::{AuthBody, generate_token};
use crate::error::Error;
use crate::model::{SignInData, TokenType, Operator};


// Simple health check endpoint
pub async fn health_check_handler() -> impl IntoResponse {
	Json(serde_json::json!({
        "status": "ok",
    }))
}

#[derive(Deserialize, Serialize)]
pub struct PingParams {
	pub data: String
}
// Simple Ping endpoint
pub async fn ping_handler(
	body: Bytes
) -> impl IntoResponse {
	body
}

/// Handler for the operator login endpoint
///
/// Accepts the operator email and password as JSON
pub async fn operator_login(
	State(state): State<Arc<AppState>>,
	Json(sign_in_data): Json<SignInData>,
) -> Result<impl IntoResponse, Error> {

	let operator = sqlx::query_as!(
	    Operator,
	    "SELECT * FROM operators WHERE email LIKE $1 LIMIT 1",
	    sign_in_data.email
	).fetch_one(&state.db)
	 .await
	 .map_err(|_| Error::WrongCredentials)?;

	if !bcrypt::verify(sign_in_data.password, &operator.password).unwrap() {
		return Err(Error::WrongCredentials)
	};
	let token = generate_token(TokenType::Operator, &operator.id, &state.keys.encoding_key)?;
	sqlx::query!("UPDATE operators SET last_login = NOW() WHERE id = $1", operator.id)
		.execute(&state.db).await.map_err(|_| Error::InternalError)?;

	tracing::info!("Operator {} just logged in.", &operator.name);
	Ok((
		[("Authorization", format!("Bearer {token}"))],
		Json(AuthBody::new(token))
	))
}

