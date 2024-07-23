use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json, Router, middleware};
use axum::routing::{get, post};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Error,
    model::{db::Agent, CreateAccountData, db::Operator, OperatorPublicInfo, OperatorRole},
    AppState,
    settings::SETTINGS,
    auth::auth_middleware
};


pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let r = &SETTINGS.http.routes;
    Router::new()
        .route(&r.operators.lookup_agent, get(lookup_agent_by_id))
        .route(&r.operators.all_agents, get(list_all_agents))
        .route(&r.operators.all_operators, get(list_all_operators))
        .route(&r.operators.new_operator, post(create_operator_account))
        .route(&r.operators.who_am_i, get(show_current_operator))
        .route(&r.operators.lookup_operator, get(lookup_operator_by_id))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware))
        .with_state(app_state)
}

/// Utility function that searches for an operator using its UUID
pub async fn query_operator_by_id(
    operator_id: &Uuid,
    db: &PgPool,
) -> Result<Operator, (StatusCode, Json<Value>)> {
    let operator = sqlx::query_as!(
        Operator,
        r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
        operator_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => (
            StatusCode::OK,
            Json(json!({"Result": format!("Operator {operator_id} not found.")})),
        ),
        _ => Error::InternalError.as_tuple(),
    })?;
    Ok(operator)
}


pub async fn lookup_operator_by_id(
    Extension(acc): Extension<Operator>,
    Path(operator_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let res = sqlx::query_as!(
        Operator,
        r#"SELECT * FROM operators WHERE id = $1 LIMIT 1"#,
        operator_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => (
            StatusCode::OK,
            Json(json!({"Result": format!("Operator {operator_id} not found")})),
        ),
        _ => Error::InternalError.as_tuple(),
    })?;

    // If we requested our own data, do not filter and return everything
    let json_response = if acc.id != operator_id {
        json!(res.public_info())
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
    let all_operators = sqlx::query_as!(Operator, r#"SELECT * FROM operators LIMIT 100"#)
        .fetch_all(&data.db)
        .await
        .map_err(|_| Error::InternalError)?
        .iter()
        .filter(|op| who.role > op.role)
        .map(|op| op.clone().public_info())
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

/// This route allows the creation of new operator accounts : `POST /operator` \
/// This function will check that the account creating the new operator is not a guest
pub async fn create_operator_account(
    Extension(op): Extension<Operator>,
    State(state): State<Arc<AppState>>,
    Json(new_op): Json<CreateAccountData>,
) -> Result<impl IntoResponse, Error> {
    if op.role < new_op.role && op.role != OperatorRole::Guest {
        return Err(Error::PermissionDenied);
    }
    if new_op.password.len() < 8 {
        return Err(Error::PasswordLength);
    }
    if sqlx::query!("SELECT id FROM operators WHERE email = $1", new_op.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| Error::InternalError)?
        .is_some()
    {
        return Err(Error::EmailExists);
    }

    let hashed =
        bcrypt::hash(new_op.password, bcrypt::DEFAULT_COST).map_err(|_| Error::InternalError)?;
    sqlx::query!(
        r#"INSERT INTO operators (name, email, password, created_by, role) VALUES
			($1, $2, $3, $4, 2)"#,
        new_op.name,
        new_op.email,
        hashed,
        op.id
    )
    .execute(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("Failed to create new operator account: {err}");
        Error::InternalError
    })?;

    Ok(Json(json!({
        "Result": format!("Account {} was created successfully", new_op.name)
    })))
}

pub async fn list_all_agents(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let all_agents = sqlx::query_as!(Agent, r#"SELECT * FROM agents LIMIT 200"#)
        .fetch_all(&data.db)
        .await
        .map_err(|_| Error::InternalError)?;
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
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => (
            StatusCode::OK,
            Json(json!({"Result": format!("Agent {agent_id} not found.")})),
        ),
        _ => Error::InternalError.as_tuple(),
    })?;
    Ok(Json(json!(agent)))
}
