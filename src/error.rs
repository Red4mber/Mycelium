use std::fmt;
use std::fmt::{Display, Formatter};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, Json<serde_json::Value>)
where
	E: std::error::Error,
{
	let err_json = json!({"Error": err.to_string()});
	(StatusCode::INTERNAL_SERVER_ERROR, Json::from(err_json))
}


// error types for auth errors
#[derive(Debug, Serialize)]
pub enum Error {
	TokenCreation,
	InvalidToken,
	InvalidAuthHeader,
	WrongCredentials,
	PermissionDenied,
	TokenExpired,
	InternalError,
}
impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Error::TokenExpired => "The token has expired",
			Error::InvalidToken => "The token provided is invalid",
			Error::TokenCreation => "Failed to create a new token",
			Error::WrongCredentials => "Wrong Credentials",
			Error::PermissionDenied => "Access Denied",
			Error::InvalidAuthHeader => "Failed to read auth header",
			_ => "Internal Server Error",
		})
	}
}
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let error_code = match self {
			Error::TokenExpired => StatusCode::UNAUTHORIZED,
			Error::InvalidToken => StatusCode::UNAUTHORIZED,
			Error::WrongCredentials => StatusCode::UNAUTHORIZED,
			Error::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
			Error::PermissionDenied => StatusCode::UNAUTHORIZED,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		};
		
		(error_code, Json(json!({"Error": self.to_string()}))).into_response()
	}
}
impl std::error::Error for Error {}
