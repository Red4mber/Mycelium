
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Wrong Credentials.")]
    WrongCredentials,
    #[error("Access denied.")]
    PermissionDenied,
    #[error("There is already an account registered with this email <{0}>.")]
    EmailExists(String),
    #[error("Internal Server Error. Check logs for details.")]
    InternalError,
    #[error("The path provided is invalid.")]
    InvalidUploadPath,
    #[error("The operator {0} does not exist.")]
    OperatorDoesNotExists(Uuid),
    #[error("Admins cannot be deleted")]
    CannotDeleteAdmins,
    #[error("{0}")]
    DatabaseError(surrealdb::Error),
    #[error("{0}")]
    GenericError(String),
    #[error("Your token has expired. Please log-in again.")]
    TokenExpired,
}
impl Error {
    pub fn get_error_code(&self) -> StatusCode {
        #[allow(unreachable_patterns)]
        match self {
            Error::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::PermissionDenied => StatusCode::UNAUTHORIZED,
            Error::EmailExists(_) => StatusCode::CONFLICT,
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidUploadPath => StatusCode::BAD_REQUEST,
            Error::OperatorDoesNotExists(_) => StatusCode::NOT_FOUND,
            Error::CannotDeleteAdmins => StatusCode::METHOD_NOT_ALLOWED,
            Error::TokenExpired => StatusCode::UNAUTHORIZED,
            _ => StatusCode::IM_A_TEAPOT
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.get_error_code(),
            // [("HeaderName", "Header value")],
            Json(json!({"Error": self.to_string()}))
        ).into_response()
    }
}
impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Error {
        Error::DatabaseError(err)
    }
}