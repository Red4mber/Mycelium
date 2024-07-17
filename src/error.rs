use axum::http::StatusCode;
use axum::Json;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, Json<serde_json::Value>)
where
	E: std::error::Error,
{
	let err_json = serde_json::json!({"Error": err.to_string()});
	(StatusCode::INTERNAL_SERVER_ERROR, Json::from(err_json))
}