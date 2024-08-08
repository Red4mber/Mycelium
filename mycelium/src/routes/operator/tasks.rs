use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::sql::Thing;
use tracing::info;
use crate::AppState;
use crate::model::TaskRecord;
use crate::model::auth::AuthData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTaskData {
	pub agent_id: String,
	pub command: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTaskRecord {
	pub command: String
}

/// Handler for the Task registration route
pub async fn new_task(
	Extension(auth): Extension<AuthData>,
	State(state): State<Arc<AppState>>,
	Json(task_data): Json<NewTaskData>,
) -> Result<impl IntoResponse, crate::error::Error> {
	info!({cmd=task_data.command}, "Operator `{}` is registering a new task for agent {}.", auth.rec.name, task_data.agent_id);
	let new_task: Vec<TaskRecord> = state.db.create("task").content(NewTaskRecord {
		command: task_data.command
	}).await?;
	
	let task_id = &new_task.first().unwrap().id;

	state.db.query("RELATE $agent_id->execute->$task_id;")
		.bind(("agent_id", Thing::from(("agent".to_string(), task_data.agent_id))))
		.bind(("task_id", task_id))
		.await?;
	
	Ok(Json(json!({ "status": "ok", "Task": new_task })))
}


/// Route used to list every agent registered
pub async fn query_all_tasks(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<TaskRecord> = state.db.select("task").await.map_err(|e| e.to_string())?;
	Ok(Json(json!(res)))
}

