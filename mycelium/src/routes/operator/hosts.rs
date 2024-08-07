use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::AppState;
use crate::model::HostRecord;


/// Route used to list every agent registered
pub async fn query_all_hosts(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<HostRecord> = state.db.select("host").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}