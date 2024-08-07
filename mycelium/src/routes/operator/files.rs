use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::AppState;
use crate::model::FileRecord;


/// Route used to query every file in the database
pub async fn query_all_files(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<FileRecord> = state.db.select("file").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}
