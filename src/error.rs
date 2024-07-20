
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::{json, Value};



#[derive(thiserror::Error, Debug, Serialize)]
pub enum Error {
	#[error("Failed to create a new token.")]
	TokenCreation,
	#[error("Invalid token provided.")]
	TokenInvalid,
	#[error("The token provided has expired.")]
	TokenExpired,
	#[error("Wrong Credentials.")]
	WrongCredentials,
	#[error("Access denied.")]
	PermissionDenied,
	#[error("The password should be at least 8 characters long.")]
	PasswordLength,
	#[error("This email has already been registered.")]
	EmailExists,
	#[error("Internal Server Error. Check logs for details.")]
	InternalError,
}
impl Error {
	pub fn get_error_code(&self) -> StatusCode {
		match self {
			Error::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
			Error::TokenInvalid => StatusCode::UNAUTHORIZED,
			Error::TokenExpired => StatusCode::UNAUTHORIZED,
			Error::WrongCredentials => StatusCode::UNAUTHORIZED,
			Error::PermissionDenied => StatusCode::UNAUTHORIZED,
			Error::PasswordLength => StatusCode::OK,
			Error::EmailExists => StatusCode::CONFLICT,
			Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
	pub fn as_tuple(&self) -> (StatusCode, Json<Value>) {
		(self.get_error_code(), Json(json!({"Error": self.to_string()})))
	}
}
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		self.as_tuple().into_response()
	}
}