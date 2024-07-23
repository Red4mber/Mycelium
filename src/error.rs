
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;


//noinspection RsDuplicateDefinitionError
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
    #[error("The path provided is invalid.")]
    InvalidUploadPath,
    #[error("The operator {0} does not exist.")]
    OperatorDoesNotExists(Uuid),
    #[error("Admins cannot be deleted")]
    CannotDeleteAdmins,
}
impl Error {
    pub fn get_error_code(&self) -> StatusCode {
        #[allow(unreachable_patterns)]
        match self {
            Error::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            Error::TokenInvalid => StatusCode::UNAUTHORIZED,
            Error::TokenExpired => StatusCode::UNAUTHORIZED,
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::PermissionDenied => StatusCode::UNAUTHORIZED,
            Error::PasswordLength => StatusCode::OK,
            Error::EmailExists => StatusCode::CONFLICT,
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidUploadPath => StatusCode::BAD_REQUEST,
            Error::OperatorDoesNotExists(_) => StatusCode::NOT_FOUND,
            Error::CannotDeleteAdmins => StatusCode::METHOD_NOT_ALLOWED,
            _ => StatusCode::IM_A_TEAPOT
        }
    }
    
    // Returns the error as a `(Error Code, Json)` tuple usable as a HTTP Response
    pub fn as_tuple_json(&self) -> (StatusCode, Json<Value>) {
        ( self.get_error_code(), Json(json!({"Error": self.to_string()})) )
    }
    pub fn as_tuple_string(&self) -> (StatusCode, String) {
        ( self.get_error_code(), self.to_string() )
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.as_tuple_json().into_response()
    }
}

impl Into<(StatusCode, Json<Value>)> for Error {
    fn into(self) -> (StatusCode, Json<Value>) { self.as_tuple_json() }
}
impl Into<(StatusCode, String)> for Error {
    fn into(self) -> (StatusCode, String) { self.as_tuple_string() }
}
