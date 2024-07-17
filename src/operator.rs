use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use uuid::Uuid;
use crate::AppState;
use crate::error::internal_error;
use crate::model::OperatorPublicInfo;


pub async fn lookup_operator_by_id(
	Path(operator_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

	let operator = sqlx::query_as!(
	    super::model::Operator,
	    r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
	    operator_id
	).fetch_one(&data.db)
	 .await
	 .map_err(internal_error)?;

	let json_response = serde_json::json!({
        "status": "ok",
		"result": operator
	});
	Ok(Json(json_response))
}


pub async fn list_all_operators(
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {


	// let all_operators = sqlx::query()


	let all_operators = sqlx::query_as!(
	    super::model::Operator,
	    r#"SELECT * FROM operators LIMIT 100"#
	).fetch_all(&data.db)
	 .await
	 .map_err(internal_error)?
		.iter()
		.map(|op| {
			let op = op.clone();
			OperatorPublicInfo {
				id: op.id,
				name: op.name,
				role: op.role,
				created_by: op.created_by,
				created_at: op.created_at,
			}
		}).collect::<Vec<OperatorPublicInfo>>();

	let json_response = serde_json::json!({
        "status": "ok",
		"result": all_operators
	});
	Ok(Json(json_response))
}