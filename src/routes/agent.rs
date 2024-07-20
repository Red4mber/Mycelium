use std::sync::Arc;
use uuid::Uuid;
use axum::{
	response::IntoResponse,
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use crate::{
	AppState,
	error::internal_error
};
use crate::model::Agent;

#[allow(non_camel_case_types, unused)]
enum CPUArch {
	x86_64, aarch64, ppc64, rv64, i386
}
// TODO to be completed

/// Describes the data sent by implants when beaconing back to the team server
struct BeaconData {
	pub host: String,
	pub architecture: CPUArch,
	pub usermame: String,
	pub path: String,
	
}







pub async fn list_all_agents(
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let all_agents = sqlx::query_as!(
	    Agent,
	    r#"SELECT * FROM agents LIMIT 200"#
	).fetch_all(&data.db)
	 .await
	 .map_err(internal_error)?;

	let json_response = serde_json::json!({
        "status": "ok",
		"result": all_agents
	});
	Ok(Json(json_response))
}

pub async fn lookup_agent_by_id(
	Path(agent_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let agent = sqlx::query_as!(
	    Agent,
	    r#"SELECT * FROM agents WHERE id = $1 LIMIT 1"#,
	    agent_id
	).fetch_one(&data.db)
	 .await
	 .map_err(internal_error)?;

	let json_response = serde_json::json!({
        "status": "ok",
		"result": agent
	});
    Ok(Json(json_response))
}