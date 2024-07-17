use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use uuid::Uuid;
use crate::AppState;
use crate::error::internal_error;

// TODO Add some authentication or something
// Just to avoid letting anyone register and lookup an agent, that'd be cool


pub async fn lookup_agent(
	Path(agent_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

	let agent = sqlx::query_as!(
	    super::schema::Agent,
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