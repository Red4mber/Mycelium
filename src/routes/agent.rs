use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::middleware::from_fn_with_state;
use axum::{Json, Router};
use axum::routing::get;
use serde_json::{json, Value};
use tracing::debug;

use crate::AppState;
use crate::authentication::auth_middleware;
use crate::model::AgentRecord;


/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/dbg", get(auth_debug))
		.route("/", get(agent_query_all))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.with_state(app_state)
}

/// Route used to list every agent registered
pub async fn agent_query_all(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<AgentRecord> = state.db.select("agent").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}


pub async fn auth_debug(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
	debug!("Ping from {addr:?}");
	Json(json!({ "status": "ok" }))
}