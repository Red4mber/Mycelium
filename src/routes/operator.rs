use std::sync::Arc;

use axum::{Extension, extract::{Path, State}, http::StatusCode, Json, response::IntoResponse};
use bcrypt::verify;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
	AppState,
	error::Error,
	auth::{AuthBody, generate_token},
	model::{CreateAccountData, OperatorRole, TokenType, Operator, OperatorPublicInfo, SignInData}
};


/// Utility function that searches for an operator using its UUID
pub async fn query_operator_by_id(
	operator_id: &Uuid,
	db: &PgPool,
) -> Result<Operator, (StatusCode, Json<Value>)> {
	let operator = sqlx::query_as!(
	    Operator,
	    r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
	    operator_id
	).fetch_one(db).await.map_err(|e| match e {
		sqlx::error::Error::RowNotFound => (
			StatusCode::OK,
			Json(json!({"Result": format!("Operator {operator_id} not found.")}))
		),
		_ => Error::InternalError.as_tuple(),
	})?;
	Ok(operator)
}

/// Utility function that maps a [Operator] struct to a [OperatorPublicInfo]
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

// GET /operator/<ID>
pub async fn lookup_operator_by_id(
	Extension(acc): Extension<Operator>,
	Path(operator_id): Path<Uuid>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
	let res = sqlx::query_as!(
        Operator,
	    r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
	    operator_id
	).fetch_one(&data.db)
	 .await
	 .map_err(|e| match e {
		sqlx::error::Error::RowNotFound => (
			StatusCode::OK, 
			Json(json!({"Result": format!("Operator {operator_id} not found")}))
		),
		_ => Error::InternalError.as_tuple(),
	})?;

	// If we requested our own data, do not filter and return everything
	let json_response = if acc.id != operator_id {
		json!(filter_private_info(&res))
	} else {
		json!(res)
	};

	Ok(Json(json_response))
}

/// Returns a list of all operators 
pub async fn list_all_operators(
	Extension(who): Extension<Operator>,
	State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
	let all_operators = sqlx::query_as!(
		Operator,
		r#"SELECT * FROM operators LIMIT 100"#
	).fetch_all(&data.db)
	 .await
	 .map_err(|_| Error::InternalError)?
		.iter()
		.filter(|op| who.role > op.role )
		.map(filter_private_info)
		.collect::<Vec<OperatorPublicInfo>>();

	let json_response = json!({
        "status": "ok",
		"result": all_operators
	});
	Ok(Json(json_response))
}

/// Shows the current operator's information
pub async fn show_current_operator(
	Extension(op): Extension<Operator>,
) -> Result<impl IntoResponse, Error> {
	Ok(Json(op.clone()))
}



/// Handler for the operator login endpoint
/// 
/// Accepts the operator email and password as JSON
pub async fn operator_login(
	State(state): State<Arc<AppState>>,
	Json(sign_in_data): Json<SignInData>,
) -> Result<impl IntoResponse, Error> {
	let operator = sqlx::query_as!(
	    Operator,
	    "SELECT * FROM operators WHERE email LIKE $1 LIMIT 1",
	    sign_in_data.email
	).fetch_one(&state.db)
	 .await
	 .map_err(|_| Error::WrongCredentials)?;

	if !verify(sign_in_data.password, &operator.password).unwrap() {
		return Err(Error::WrongCredentials)
	};
	let ttl = &state.cfg.tokens.ttl;
	let token = generate_token(TokenType::Operator, &operator.id, &state.keys.encoding_key, ttl.clone())?;
	sqlx::query!("UPDATE operators SET last_login = NOW() WHERE id = $1", operator.id)
		.execute(&state.db).await.map_err(|_| Error::InternalError)?;

	tracing::info!("Operator {} just logged in.", &operator.name);
	Ok((
		[("Authorization", format!("Bearer {token}"))], 
		Json(AuthBody::new(token))
	))
}


/// This route allows the creation of new operator accounts : `POST /operator` \
/// This function will check that the account creating the new operator is not a guest
pub async fn create_operator_account(
	Extension(op): Extension<Operator>,
	State(state): State<Arc<AppState>>,
	Json(new_op): Json<CreateAccountData>,
) -> Result<impl IntoResponse, Error> {

	if op.role < new_op.role && op.role != OperatorRole::Guest { 
		return Err(Error::PermissionDenied) 
	}
	if new_op.password.len() < 8 { return Err(Error::PasswordLength) }
	if sqlx::query!("SELECT id FROM operators WHERE email = $1", new_op.email)
		.fetch_optional(&state.db).await.map_err(|_| Error::InternalError)?.is_some() {
		return Err(Error::EmailExists)
	}
	
	let hashed = bcrypt::hash(new_op.password, bcrypt::DEFAULT_COST).map_err(|_| Error::InternalError)?;
	sqlx::query!(r#"INSERT INTO operators (name, email, password, created_by, role) VALUES
			($1, $2, $3, $4, 2)"#, new_op.name, new_op.email, hashed, op.id)
		.execute(&state.db).await.map_err(|err| {
			tracing::error!("Failed to create new operator account: {err}");
			Error::InternalError
		})?;

	Ok(Json(json!({
		"Result": format!("Account {} was created successfully", new_op.name)
	})))
}