use std::sync::Arc;

use axum::{
	extract::{Path, State, Request},
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
use crate::auth::{AuthBody, generate_token};
use crate::error::{Error};
use crate::model::TokenType;


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

// Utility function that maps a Operator struct to a OperatorPublicInfo
fn filter_private_info(op: &Operator) -> OperatorPublicInfo {
	let op = op.clone();
	OperatorPublicInfo {
		id: op.id,
		name: op.name,
		role: op.role,
		created_by: op.created_by,
		created_at: op.created_at,
	}
}

pub async fn lookup_operator_by_id(
	Path(operator_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
	req: Request
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let res = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
	    operator_id
	).fetch_one(&data.db)
	 .await
	 .map_err(internal_error)?;

	let current_op = req
		.extensions()
		.get::<Operator>()
		.expect("Operator should be logged in");
	
	let json_response = if current_op.id != operator_id {
		json!({
	        "status": "ok",
			"result": filter_private_info(&res)
		})
	} else { 
		json!({
	        "status": "ok",
			"result": res
		})
	};

	Ok(Json(json_response))
}

pub async fn list_all_operators(
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let all_operators = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators LIMIT 100"#
	).fetch_all(&data.db)
	 .await
	 .map_err(internal_error)?
		.iter()
		.map(|op| {
			filter_private_info(op)
		}).collect::<Vec<OperatorPublicInfo>>();

	let json_response = json!({
        "status": "ok",
		"result": all_operators
	});
	Ok(Json(json_response))
}

pub async fn show_current_operator(
	req: Request
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let op = match req.extensions().get::<Operator>() {
		Some(op) => {
			op.clone()
		},
		None => return Err((
			StatusCode::INTERNAL_SERVER_ERROR,
			Json::from(json!({"Error": "Internal error"}))
		))
	};
	Ok(Json(json!(op)))
}



/// Handler for the operator login endpoint
/// 
/// Accepts the operator email and password as JSON
pub async fn operator_login(
	State(state): State<Arc<AppState>>,
	Json(sign_in_data): Json<OperatorSignInData>,
) -> Result<Json<AuthBody>, Error> {
	let operator = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators WHERE email LIKE $1 LIMIT 1"#,
	    sign_in_data.email
	).fetch_one(&state.db)
	 .await
	 .map_err(|_| Error::WrongCredentials)?;

	if !verify(sign_in_data.password, &operator.password).unwrap() {
		return Err(Error::WrongCredentials)
	};
	
	let token = generate_token(TokenType::Operator, &operator.id, &state.encoding_key)?;
	
	tracing::info!("Operator {} just logged in.", &operator.name);
	Ok(Json(AuthBody::new(token)))
	
}

