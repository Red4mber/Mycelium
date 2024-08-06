use std::net::SocketAddr;
use std::sync::Arc;
use serde_json::json;
use tracing::debug;

use axum::{
	routing::{get, post},
	response::IntoResponse,
	body::Bytes, Json, Router,
	extract::ConnectInfo,
};
use crate::AppState;

/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/ping", get(healthcheck_handler))
		.route("/ping", post(ping_handler))
		.with_state(app_state)
}


/// Healthcheck endpoint, always returns `{ "status": "ok" }`
async fn healthcheck_handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
	debug!(from=?addr, "Ping Received");
	Json(json!({ "status": "ok" }))
}

/// Simple Ping API endpoint - Respond with the data it receives
async fn ping_handler(
	ConnectInfo(addr): ConnectInfo<SocketAddr>, body: Bytes
) -> impl IntoResponse {
	debug!(request_body=?body.clone(), from=?addr, "Ping Received");
	body
}
