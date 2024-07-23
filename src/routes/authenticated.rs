use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json, Router, middleware};
use axum::routing::{delete, get, post};
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
use crate::routes::agents::query_agent_by_id;


pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let r = &SETTINGS.http.routes;
    Router::new()
        .route(&r.operators.lookup_agent, get(lookup_agent_by_id))
        .route(&r.operators.all_agents, get(list_all_agents))
        .route(&r.operators.all_operators, get(list_all_operators))
        .route(&r.operators.new_operator, post(create_operator_account))
        .route(&r.operators.del_operator, delete(delete_operator_account))
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
        .map_err(|_| Error::InternalError.as_tuple_json())?;

    match result {
        Some(row) => Ok(Operator::from(row)),
        None => {
            Err((StatusCode::NO_CONTENT, Json(json!({"Result": format!("Operator {operator_id} not found.")})) ))
        }
    }
}



pub async fn delete_operator_account(
    Extension(op): Extension<Operator>,
    Path(operator_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    if op.role == OperatorRole::Guest {
        tracing::warn!("Guest `{}` tried to delete operator `{operator_id}` !", &op.id);
        return Err(Error::PermissionDenied.into());
    }
    tracing::trace!("Operator `{}` is trying to delete operator `{operator_id}`", op.name);
    let result = state.db.query_opt("SELECT * FROM operators WHERE id = $1 LIMIT 1", &[&operator_id])
                      .await.map_err(|_| Error::InternalError.into())?;
    if result.is_none() { return Err(Error::OperatorDoesNotExists(operator_id).into()) }
    
    if let Some(row) = result {
        let _ = match Operator::from(row).role {
            OperatorRole::Admin => Err(Error::CannotDeleteAdmins.into()),
            _ => Ok("")
        }?;
    }
    
    state.db
         .execute(r#"DELETE FROM operators WHERE id = $1"#, &[&operator_id])
         .await.map_err(|err| {
        tracing::error!("Failed to delete operator: `{err}`");
        Error::InternalError.into()
    })?;

    tracing::trace!("Account `{operator_id}` was successfully deleted [by Operator `{}`]", op.name);
    Ok(Json(json!({
        "Result": format!("Account `{}` was successfully deleted", operator_id)
    })))
}





pub async fn lookup_operator_by_id(
    Extension(op): Extension<Operator>,
    Path(operator_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    tracing::trace!("Operator `{}` is looking for operator `{operator_id}`", op.name);
    let res = query_operator_by_id(&operator_id, &state.db).await?;

    // If we requested our own data, do not filter and return everything
    let json_response = if op.id != operator_id {
        json!(res.public_info())
    } else {
        json!(res)
    };

    Ok(Json(json_response))
}


/// Returns a list of all operators
pub async fn list_all_operators(
    Extension(op): Extension<Operator>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::trace!("Operator `{}` is listing all operator accounts !", op.name);
    if op.role == OperatorRole::Guest { return Err(Error::PermissionDenied) }
    let all_operators = state.db.query("SELECT * FROM operators", &[])
        .await
        .map_err(|_| Error::InternalError)?
        .iter()
        .map(|row| Operator::from(row.to_owned()) )
        .map(|o| o.clone().public_info())
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
    if op.role == OperatorRole::Guest {
        tracing::warn!("Guest `{}` [{}] tried to create a new account !", &op.name, &op.id);
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
    
    tracing::info!("Operator `{}` just created a new account : {:?} - {} <{}>", &op.name, &new_op.role, &new_op.name, &new_op.email );
    Ok(Json(json!({
        "Result": format!("Account {} was created successfully", new_op.name)
    })))
}

pub async fn list_all_agents(
    Extension(op): Extension<Operator>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::trace!("Operator `{}` is listing all agents !", op.name);
    let all_agents = state.db.query("SELECT * FROM agents", &[])
        .await
        .map_err(|_| Error::InternalError)?
        .iter()
        .map(|row| Agent::from(row.to_owned()) )
        .collect::<Vec<Agent>>();
    Ok((StatusCode::OK, Json(json!(all_agents))))

}

pub async fn lookup_agent_by_id(
    Extension(op): Extension<Operator>,
    Path(agent_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    tracing::trace!("Operator `{}` is looking for agent `{agent_id}`", op.name);
    let res = query_agent_by_id(&agent_id, &state.db).await?;
    tracing::trace!("{:#?}", &res);
    Ok(Json(json!(res)))
}
