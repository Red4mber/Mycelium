use std::sync::Arc;
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::{Json, Router};
use axum::routing::get;
use serde_json::{json, Value};
use crate::AppState;
use crate::authentication::middleware::auth_middleware;
use crate::model::OperatorRecord;


/// Returns a router with all the routes of this module
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", get(operator_query_all))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.with_state(app_state)
}

/// Route used to list every operator in the database
pub async fn operator_query_all(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<OperatorRecord> = state.db.select("operator").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}
