use std::sync::Arc;
use uuid::Uuid;
use axum::{
	response::IntoResponse,
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use serde_json::{json, Value};
use crate::{error::Error, AppState};
use crate::model::Agent;

pub async fn list_all_agents(
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
	let all_agents = sqlx::query_as!(
	    Agent,
	    r#"SELECT * FROM agents LIMIT 200"#
	).fetch_all(&data.db).await.map_err(|_| Error::InternalError)?;
	Ok(Json(json!(all_agents)))
}

pub async fn lookup_agent_by_id(
	Path(agent_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let agent = sqlx::query_as!(
	    Agent,
	    r#"SELECT * FROM agents WHERE id = $1 LIMIT 1"#,
	    agent_id
	).fetch_one(&data.db)
	 .await.map_err(|e| match e {
		sqlx::error::Error::RowNotFound => (
			StatusCode::OK,
			Json(json!({"Result": format!("Agent {agent_id} not found.")}))
		),
		_ => Error::InternalError.as_tuple(),
	})?;
    Ok(Json(json!(agent)))
}