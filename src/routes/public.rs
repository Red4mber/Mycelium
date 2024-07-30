// use std::string::String;
use std::net::SocketAddr;
use std::sync::Arc;
use serde_json::json;
use surrealdb::opt::auth::{Record, Root};
use tracing::{debug, error, info};
use serde::{Deserialize, Serialize};

use axum::{
	routing::{get, post},
	response::IntoResponse,
	body::Bytes, Json, Router,
	extract::{ConnectInfo, State},
	http::{header, StatusCode}
};
use crate::{
	error::Error,
	AppState,
	settings::CFG
};

/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/ping", get(healthcheck_handler))
		.route("/ping", post(ping_handler))
		.route("/login", post(signin_handler))
		.with_state(app_state)
}


/// Healthcheck endpoint, always returns `{ "status": "ok" }`
pub async fn healthcheck_handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
	debug!(from=?addr, "Ping Received");
	Json(json!({ "status": "ok" }))
}

/// Simple Ping API endpoint - Respond with the data it receives
pub async fn ping_handler(
	ConnectInfo(addr): ConnectInfo<SocketAddr>, body: Bytes
) -> impl IntoResponse {
	debug!(request_body=?body.clone(), from=?addr, "Ping Received");
	body
}

/// Parameters of the Sign-In route
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
	email: String,
	pass: String,
}


/// Handler for the operator Sign-In route
pub async fn signin_handler(
	State(state): State<Arc<AppState>>,
	Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, Error> {
	state.db.signin(Root {
		username: &CFG.db.user,
		password: &CFG.db.pass,
	}).await?;
	state.db.use_ns(&CFG.db.ns)
	        .use_db(&CFG.db.db)
	        .await?;
	
	
	let auth_data = Record {
		namespace: &CFG.db.ns,
		database: &CFG.db.db,
		params: creds.clone(),
		access: "operator",
	};
	// Wrong method : The access method does not exist
	// ??????? : The access method cannot be used in the requested operation
	
	// let dbg = (creds.params.email.clone(), creds.params.pass.clone()); // For debug purposes
	let res = state.db.signin(auth_data).await.map_err(|err| {
		error!(credentials=?&creds, "Failed login attempt.");
		Error::DatabaseError(err)
	})?;
	info!(user=&creds.email,"Successfully logged in.");
	
	let token = res.as_insecure_token();
	Ok((
		StatusCode::OK, 
		[(header::AUTHORIZATION, format!("Bearer {}", token))],
		Json(json!({ "Result":"Successfully logged in", "token": token }))
	))
}

// DEBUG ROUTE
// 
// /// Holds the data needed by the debug routes
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[cfg(debug_assertions)]
// pub struct DebugData {
// 	// pub name: String,
// 	pub email: String,
// 	pub pass: String,
// }
// /// Handler for the debug routes,
// #[axum_macros::debug_handler]
// #[cfg(debug_assertions)]
// pub async fn public_debug(
// 	State(state): State<Arc<AppState>>,
// 	Json(post_data): Json<DebugData>,
// ) -> Result<impl IntoResponse, Error> {
// 	debug!("Data Received :\n{}", format!("{:#?}", post_data));
// 	state.db.use_ns(&SETTINGS.db_params.ns)
// 			.use_db(&SETTINGS.db_params.db)
// 			.await.map_err(Error::Database)?;
// 	
// 	state.db.signin(Root {
// 		username: &SETTINGS.db_params.username,
// 		password: &SETTINGS.db_params.password,
// 	}).await.map_err(Error::Database)?;
// 	
// 
// 	let res = state.db.signin(Record { 
// 		namespace: &SETTINGS.db_params.ns,
// 		database: &SETTINGS.db_params.db,
// 		access: "operator_access",
// 		params: post_data.clone() }).await;
// 
// 	// Check if the password is correct with a different query
// 	let sql = "RETURN crypto::argon2::compare((SELECT pass FROM ONLY operator where email = $email LIMIT 1).pass, $pass)";
// 	let mut response = state.db
// 		.query(sql)
// 		.bind(("email", post_data.email))
// 		.bind(("pass", post_data.pass))
// 		.await.map_err(Error::Database)?;
// 	
// 	// Testing if passwords are matching
// 	let test: Option<bool> = response.take(0).map_err(Error::Database)?;
// 	match test { Some(b) => { debug!("Passwords match ? {b:?}"); }
// 				  None => { error!("Shit's fucked up") } };
// 	
// 	// Print results and stuff
// 	match res { Ok(token) => { Ok(Json(json!({ "token": token }))) },
// 				Err(e) => { error!("{e:?}"); Err(Error::Database(e)) } }
// }
// 
