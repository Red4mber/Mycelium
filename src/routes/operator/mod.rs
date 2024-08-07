use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{ConnectInfo, State};
use axum::middleware::from_fn_with_state;
use axum::{Json, Router};
use axum::body::Bytes;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::{json, Value};
use tracing::debug;

use crate::AppState;
use crate::authentication::{auth_middleware};
use crate::model::OperatorRecord;

pub mod auth;
mod files;
mod hosts;
mod agents;

/// Returns a router with all the routes of this module
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/operator/all", get(query_all_operators))
		.route("/host/all", get(hosts::query_all_hosts))
		.route("/file/all", get(files::query_all_files))
		.route("/agent/all", get(agents::query_all_agents))
		.route("/agent/new", post(agents::new_agent))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.route("/jwks", get(auth::jwks_handler))
		.route("/login", post(auth::login_handler))
		.route("/ping", get(healthcheck_handler))
		.route("/ping", post(echo_handler))
		.with_state(app_state)
}

/// Route used to list every operator in the database
async fn query_all_operators(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<OperatorRecord> = state.db.select("operator").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}

/// Healthcheck endpoint, always returns `{ "status": "ok" }`
async fn healthcheck_handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
	debug!(from=?addr, "Ping Received");
	Json(json!({ "status": "ok" }))
}

/// Simple Ping API endpoint - Respond with the data it receives
async fn echo_handler(
	ConnectInfo(addr): ConnectInfo<SocketAddr>, body: Bytes
) -> impl IntoResponse {
	debug!(request_body=?body.clone(), from=?addr, "Ping Received");
	body
}
