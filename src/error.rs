use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

// Hi future me :D
// TODO = Rewrite this entire file
// like seriously why is there two different auth error types ?
// and isn't AuthError just a generic http error ?

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, Json<serde_json::Value>)
where
	E: Error,
{
	let err_json = serde_json::json!({"Error": err.to_string()});
	(StatusCode::INTERNAL_SERVER_ERROR, Json::from(err_json))
}


#[derive(Debug, Clone)]
pub struct AuthError {
	pub message: String,
	pub status_code: StatusCode
}
impl Display for AuthError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}] {}", self.status_code.as_str(), self.message)
	}
}
impl Error for AuthError {}
impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		(self.status_code, self.message).into_response()
	}
}








// error types for auth errors
#[derive(Debug, Serialize)]
pub enum AuthErrorType {
	TokenCreation,
	InvalidToken,
	WrongCredentials,
	TokenExpired,
}
impl Display for AuthErrorType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let str = match self {
			AuthErrorType::TokenExpired => "The token has expired",
			AuthErrorType::InvalidToken => "The token provided is invalid",
			AuthErrorType::TokenCreation => "Failed to create a new token",
			AuthErrorType::WrongCredentials => "Wrong Credentials",
		};
		f.write_str(str)
	}
}
impl Error for AuthErrorType {}
