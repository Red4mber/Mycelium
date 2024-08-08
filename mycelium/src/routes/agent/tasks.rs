use std::str::FromStr;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use serde::Deserialize;
use serde_json::{json, Value};
use serde_with::serde_derive::Serialize;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use tracing::{debug, info, trace};
use crate::{AppState, CFG, Error};
use crate::authentication::agent::AgentData;
use crate::Error::InternalError;
use crate::model::{AgentTasks, TaskRecord, TaskStatus, TimeRecord};

#[derive(Serialize)]
struct TaskData {
	task_id: String,
	command: String,
}


pub async fn task_poll_handler(
	Extension(auth): Extension<AgentData>,
	State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, Error> {
	// trace!({id=auth.record.id.to_string()}, "Agent is polling pending tasks.");
	let mut res = state.db
		.query("SELECT ->execute->task FROM $agent;")
		.bind(("agent", auth.record.id.clone()))
		.await?;
	let tasks: Option<AgentTasks> = res.take("->execute")?;
	let tasklist = tasks.unwrap().task;

	if tasklist.is_empty() {
		return Ok(Json(json!([])))
	}
	let mut pending_tasks: Vec<TaskData> = Vec::new();
	for task_id in tasklist {
		let rec: Option<TaskRecord> = state.db.select(task_id).await?;
		if let TaskStatus::Pending = rec.clone().unwrap().status {
			let task = rec.unwrap();
			pending_tasks.push(TaskData {
				task_id: task.id.to_string(),
				command: task.command,
			});
		}
	}

	Ok(Json(json!(pending_tasks)))
}

#[derive(Deserialize, Clone, Debug)]
pub struct TaskUpdateData {
	pub id: String,
	pub output: String,
	pub status: String,
}
#[derive(Serialize)]
pub struct TaskUpdateRecord {
	pub output: String,
	pub status: TaskStatus,
}
impl From<TaskUpdateData> for TaskUpdateRecord {
	fn from(data: TaskUpdateData) -> Self {
		TaskUpdateRecord {
			output: data.output,
			status: match data.status.as_str() {
				"Error" => TaskStatus::Error,
				"Running" => TaskStatus::Running,
				"Success" => TaskStatus::Success,
				_ => TaskStatus::Pending
			},
		}
	}
}

pub async fn task_update_handler(
	State(state): State<Arc<AppState>>,
	Json(task_data): Json<TaskUpdateData>,
) -> Result<(), Error> {
	info!({task=?task_data}, "Task update received !");
	let task_id = Thing::from_str(task_data.id.clone().as_str()).map_err(|_| InternalError)?;
	let _res: Option<TaskRecord> = state.db
		.update(task_id)
		.merge(TaskUpdateRecord::from(task_data))
		.await?;
	Ok(())
}