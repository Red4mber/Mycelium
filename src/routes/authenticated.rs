use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json, Router, middleware};
use axum::routing::{get, post};
use serde_json::{json, Value};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::{
    error::Error,
    model::{db::{Agent, OperatorRole}, CreateAccountData, db::Operator, OperatorPublicInfo},
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
    db: &Client,
) -> Result<Operator, (StatusCode, Json<Value>)> {
    let result = db
        .query_opt("SELECT * FROM operators WHERE id = $1 LIMIT 1", &[&operator_id])
        .await
        .map_err(|_| Error::InternalError.as_tuple())?;
    
    match result {
        Some(row) => Ok(Operator::from(row)),
        None => {
            Err(( StatusCode::OK,  Json(json!({"Result": format!("Operator {operator_id} not found.")})) ))
        }
    }
}

pub async fn lookup_operator_by_id(
    Extension(acc): Extension<Operator>,
    Path(operator_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
   let res = query_operator_by_id(&operator_id, &state.db).await?;

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
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    if who.role == OperatorRole::Guest { return Err(Error::PermissionDenied) }
    let all_operators = state.db.query("SELECT * FROM operators", &[])
        .await
        .map_err(|_| Error::InternalError)?
        .iter()
        .map(|row| Operator::from(row.to_owned()) )
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
    if op.role != OperatorRole::Guest {
        return Err(Error::PermissionDenied);
    }
    if new_op.password.len() < 8 { return Err(Error::PasswordLength); }

    let result = state.db
          .query_opt("SELECT * FROM operators WHERE email = $1 LIMIT 1", &[&new_op.email])
          .await.map_err(|_| Error::InternalError)?;
    if result.is_some() { return Err(Error::EmailExists) }
    
    let hashed =
        bcrypt::hash(new_op.password, bcrypt::DEFAULT_COST).map_err(|_| Error::InternalError)?;
    
    state.db
          .execute(r#"INSERT INTO operators (name, email, password, created_by, role) 
           VALUES ($1, $2, $3, $4, $5)"#, 
           &[ &new_op.name, &new_op.email, &hashed, &op.id, &new_op.role ])
        .await.map_err(|err| {
            tracing::error!("Failed to create new operator account: {err}");
            Error::InternalError
        })?;

    Ok(Json(json!({
        "Result": format!("Account {} was created successfully", new_op.name)
    })))
}

pub async fn list_all_agents(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let all_agents = state.db.query("SELECT * FROM agents", &[])
        .await
        .map_err(|_| Error::InternalError)?
        .iter()
        .map(|row| Agent::from(row.to_owned()) )
        .collect::<Vec<Agent>>();
    Ok((StatusCode::OK, Json(json!(all_agents))))

}

pub async fn lookup_agent_by_id(
    Path(agent_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let res = query_operator_by_id(&agent_id, &state.db).await?;

    Ok(Json(json!(res)))
}
