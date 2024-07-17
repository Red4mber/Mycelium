use std::net::IpAddr;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::DatabaseConnection;
use crate::db_models::Agent;
use crate::error::internal_error;


// TODO Add some authentication or something
// Just to avoid letting anyone register an agent, that'd be cool

pub struct AgentRegistrationRequest {
	id: Uuid
}

pub async fn query_agent(
	Path(agent_id): Path<Uuid>,
	DatabaseConnection(conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, String)> {

	// let row = conn.query_one(r#"SELECT * FROM agent WHERE title LIKE $1"#, &[])
	// 	.bind(agent_id.hyphenated().to_string())
	// 	.await
	// 	.map_err(internal_error)?;

	// let agent: Agent = row.try_get(0).map_err(internal_error);

	// let results = sqlx::query_as!(
	// 	Agent,
	// 	r#"SELECT * FROM agent WHERE title LIKE $1"#,
	// 	agent_id.hyphenated().to_string()
	// )
	// 	.fetch_one(&conn)
	// 	.await?;

	let json_response = serde_json::to_string(row.try_get::<Agent>(0).map_err(internal_error));

	Ok(Json(json_response))
}




// we can extract the connection pool with `State`
// pub async fn register_new_agent(
// 	Json(body): Json<AgentRegistrationRequest>,
//     DatabaseConnection(mut conn): DatabaseConnection,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
// 	let agent_id = body.id.hyphenated().to_string();
//
// 	let res = sqlx::query_scalar("SELECT * FROM agents WHERE id = $1")
// 		.bind(&agent_id)
// 		.fetch_one(&mut *conn)
// 		.await.map_err(internal_error);
//
// 	let agent = match res {
// 		Ok(agent) => {},
// 		Err(sqlx::Error::RowNotFound) => {
//
// 		},
// 		Err(e) => {
//
// 		}
//
// 	}
//
// 	// match res {
// 	// 	Ok(Some(agents)) => {
// 	// 		Err((StatusCode::CONFLICT, format!("Agent {agent_id} already registered !")))
// 	// 	},
// 	// 	Ok(None) => {
// 	// 		Ok(Json(serde_json::json!({
// 	// 	        "status": "ok",
// 	// 	        "id": agent_id,
// 	// 	        "token": "aaaaaaaaaaaaaaaaaaaa"
// 	// 	    })))
// 	// 	}
// 	// }
//
//
//
// }
// async fn db_test(
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&mut *conn)
//         .await
//
// }






pub async fn list_agents() -> impl IntoResponse {
	// const MESSAGE: &str = "Mycelium API";
	let json_response = serde_json::json!({
        "status": "ok",
        "agents": []
    });
	Json(json_response)
}


