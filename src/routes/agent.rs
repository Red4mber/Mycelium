use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::middleware::from_fn_with_state;
use axum::{Extension, Json, Router};
use axum::routing::{get, post};
use serde_json::{json, Value};
use surrealdb::sql::Thing;
use tracing::info;

use crate::AppState;
use crate::authentication::{agent_middleware, auth_middleware};
use crate::model::AgentRecord;
use crate::model::auth::AuthData;


/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", get(agent_query_all))
		.route("/new", post(new_agent))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.route("/", post(beacon_handler).layer(from_fn_with_state(app_state.clone(), agent_middleware)))
		.with_state(app_state)
}

/// Route used to list every agent registered
async fn agent_query_all(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<AgentRecord> = state.db.select("agent").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}


/// Handler for the agent registering route
async fn new_agent(
	Extension(auth): Extension<AuthData>,
	State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, crate::error::Error> {
	info!({operator_id=auth.rec.id.to_string()}, "Operator `{}` is registering a new agent.", auth.rec.name);
	let new_id = uuid::Uuid::new_v4().to_string();
	let new_agent: Option<AgentRecord> = state.db.insert(("agent", new_id.clone())).await?;
	state.db.query("RELATE $operator_id->control->$agent_id;")
		.bind(("operator_id", auth.rec.id))
		.bind(("agent_id", Thing::from(("agent".to_string(), new_id))))
		.await?;
	Ok(Json(json!({ "status": "ok", "Agent": new_agent })))
}


async fn beacon_handler() {
	
}