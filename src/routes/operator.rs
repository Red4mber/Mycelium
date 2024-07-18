use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json,
	response::IntoResponse,
};
use bcrypt::verify;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
	AppState,
	error::internal_error,
	model::{Operator, OperatorPublicInfo, OperatorSignInData}
};
use crate::auth::AuthBody;
use crate::error::AuthErrorType;


pub async fn query_operator_by_id(
	operator_id: &Uuid,
	db: &PgPool,
) -> Result<Operator, (StatusCode, Json<Value>)> {
	let operator = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
	    operator_id
	).fetch_one(db)
	 .await
	 .map_err(internal_error)?;
	Ok(operator)
}

pub async fn lookup_operator_by_id(
	Path(operator_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

	let operator = sqlx::query_as!(
	    Operator,
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
	let all_operators = sqlx::query_as!(
	    Operator,
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

pub async fn operator_login(
	State(state): State<Arc<AppState>>,
	Json(sign_in_data): Json<OperatorSignInData>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let operator = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators WHERE email LIKE $1 LIMIT 1"#,
	    sign_in_data.email
	).fetch_one(&state.db)
	 .await
	 .map_err(internal_error)?;

	if !verify(sign_in_data.password, &operator.password).unwrap() {
		return Err((StatusCode::UNAUTHORIZED, Json(json!(AuthErrorType::WrongCredentials))))
	};
	let token = crate::auth::generate_token(&operator.id, &state.encoding_key).map_err(internal_error)?;
	tracing::info!("Operator {} just logged in.", &operator.name);
	Ok( Json( AuthBody::new(token) ) )
}

