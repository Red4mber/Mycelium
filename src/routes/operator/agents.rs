use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use serde_json::{json, Value};
use surrealdb::sql::Thing;
use tracing::info;

use crate::AppState;
use crate::model::AgentRecord;
use crate::model::auth::AuthData;


/// Route used to list every agent registered
pub async fn query_all_agents(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<AgentRecord> = state.db.select("agent").await.map_err(|e| e.to_string())?;
	Ok(Json(json!(res)))
}


/// Handler for the agent registering route
pub async fn new_agent(
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
