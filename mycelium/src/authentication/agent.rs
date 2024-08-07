use std::str::FromStr;
use std::sync::Arc;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use axum::extract::{Request, State};
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::error;
use uuid::Uuid;
use crate::{AppState, CFG, Error};
use crate::Error::InternalError;
use crate::model::AgentRecord;

#[derive(Clone, Debug)]
pub struct AgentData {
	pub record: AgentRecord
}


pub async fn agent_middleware(
	State(state): State<Arc<AppState>>,
	mut req: Request, next: Next,
) -> Result<Response, Error> {
	// TODO = Encryption or something
	state.db.signin(Root {
		username: &CFG.db.user,
		password: &CFG.db.pass,
	}).await?;
	state.db.use_ns(&CFG.db.ns)
	     .use_db(&CFG.db.db)
	     .await?;

	// Extract the agent UUID from the Authorization header
	let auth_header = match req.headers_mut().get(http::header::AUTHORIZATION) {
		Some(val) => val.to_str().map_err(|_| Error::WrongCredentials)?,
		None => return Err(Error::PermissionDenied),
	};
	let mut header = auth_header.split_whitespace();
	let uuid = header.next().ok_or(Error::PermissionDenied)?;
	let uuid: Uuid = Uuid::from_str(uuid).map_err(|_| Error::PermissionDenied)?;

	let record = get_agent_record(&Thing::from_str(format!("agent:`{uuid}`").as_str()).map_err(|_| InternalError)?, &state.db).await?;
	
	req.extensions_mut().insert(AgentData {
		record
	});
	Ok(next.run(req).await)
}


pub async fn get_agent_record(agent_id: &Thing, db: &Surreal<Any>) -> Result<AgentRecord, Error> {
	let response: Option<AgentRecord> = db.select(agent_id).await?;
	response.ok_or_else(|| {
		error!(record=agent_id.to_string(), "Can't find the Agent in the database.");
		InternalError
	})
}