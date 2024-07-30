#![allow(unused)]


use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use futures::TryFutureExt;
use jsonwebtoken::{Algorithm, decode, DecodingKey, TokenData, Validation};
use surrealdb::engine::any::Any;
use surrealdb::iam::token::Claims;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
pub use tracing::{debug, error};
use tracing::info;

use Error::EmailExists;

use crate::{AppState, CFG};
use crate::error::Error;
use crate::model::{AgentRecord, OperatorRecord};

// /// Data added to the HTTP request the authentication middleware to identify the author of the request
// #[derive(Debug, Clone)]
// pub enum AuthData {
// 	Operator(OperatorAuth),
// 	Agent(AgentAuth)
// }


#[derive(Debug, Clone)]
pub struct AuthData {
	/// Contains all the Claims decoded from the JWT
	jwt: TokenData<Claims>,
	/// Contains the database record of the current user
	rec: OperatorRecord
}



// Todo REWORK AGENT AUTHENTICATION // Think before you do
// Plan stuff, design the system before starting it
// Consider a TOTP rather than a token


// Todo rewrite doc
/// JWT Authentication middleware
///
/// Reads a JWT token from the `Authorization` Header \
/// Check its Claims to verify the use can access this page \
/// Returns an error and blocks the request if the token is invalid,  //InvalidSignature //InvalidKeyFormat
pub async fn auth_middleware(
	State(state): State<Arc<AppState>>,
	mut req: Request, next: Next,
) -> Result<Response, Error> {
	// 
	state.db.signin(Root {
		username: &CFG.db.user,
		password: &CFG.db.pass,
	}).await?;
	state.db.use_ns(&CFG.db.ns)
		.use_db(&CFG.db.db)
		.await?;

	// Extract the token from the Authorization header
	let auth_header = match req.headers_mut().get(http::header::AUTHORIZATION) {
		Some(val) => val.to_str().map_err(|_| Error::WrongCredentials)?,
		None => return Err(Error::PermissionDenied),
	};
	let mut header = auth_header.split_whitespace();
	let (_, token) = (header.next(), header.next().ok_or(Error::PermissionDenied)?);

	// Once the token has been parsed, we try to authenticate with it to validate it
	// state.db.authenticate(token).await?;
	// let token_data = decode_token(token.to_string()).await.map_err(|err| {
	// 	error!(err=err, token=token.to_string(), "Failed to decode token.");
	// 	Error::InternalError
	// })?;

	// Query operator record from the database to make sure the user exists
	// let record_id = token_data.clone().claims.id.ok_or(Error::TokenInvalid)?;
	// let thing = Thing::from_str(&record_id).unwrap();
	// let auth_data = match thing.tb.as_str() {
	// 	"operator" => {
	// 		Ok(AuthData::Operator(OperatorAuth {
	// 			jwt: token_data,
	// 			rec: get_operator_record(&thing, &state.db).await?,
	// 		}))
	// 	},
	// 	"agent" => {
	// 		Ok(AuthData::Agent(AgentAuth {
	// 			jwt: token_data,
	// 			rec: get_agent_record(&thing, &state.db).await?
	// 		}))
	// 	},
	// 	_ => Err(Error::InternalError)
	// }?;

	// req.extensions_mut().insert(auth_data);
	Ok(next.run(req).await)
}


pub async fn get_operator_record(operator_id: &Thing, db: &Surreal<Any>) -> Result<OperatorRecord, Error> {
	let response: Option<OperatorRecord> = db.select(operator_id).await?;
	response.ok_or_else(|| {
		error!(record=operator_id.to_string(), "Can't find the record in the database."); 
		Error::InternalError
	})
}

pub async fn get_agent_record(agent_id: &Thing, db: &Surreal<Any>) -> Result<AgentRecord, Error> {
	let response: Option<AgentRecord> = db.select(agent_id).await?;
	response.ok_or_else(|| {
		error!(record=agent_id.to_string(), "Can't find the Agent in the database.");
		Error::InternalError
	})
}