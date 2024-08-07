use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::{json, Value};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use tracing::info;

use crate::{AppState, CFG};
use crate::authentication::{agent_middleware, auth_middleware};
use crate::authentication::agent::AgentData;
use crate::model::{AgentRecord, BeaconData, CPUArch, HostRecord, HostTarget, NoIdAgentRecord, OSInfo};
use crate::model::auth::AuthData;


/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", get(agent_query_all))
		.route("/new", post(new_agent))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.route("/", post(beacon_handler).layer(from_fn_with_state(app_state.clone(), agent_middleware)))
		.with_state(app_state)
}


/// Route used to list every agent registered
async fn agent_query_all(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<AgentRecord> = state.db.select("agent").await.map_err(|e| e.to_string())?;
	Ok(Json(json!(res)))
}


/// Handler for the agent registering route
async fn new_agent(
	Extension(auth): Extension<AuthData>,
	State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, crate::error::Error> {
	info!({operator_id=auth.rec.id.to_string()}, "Operator `{}` is registering a new agent.", auth.rec.name);
	let new_id = uuid::Uuid::new_v4().to_string();
	let new_agent: Option<AgentRecord> = state.db.insert(("agent", new_id.clone())).await?;
	state.db.query("RELATE $operator_id->control->$agent_id;")
		.bind(("operator_id", auth.rec.id))
		.bind(("agent_id", Thing::from(("agent".to_string(), new_id))))
		.await?;
	Ok(Json(json!({ "status": "ok", "Agent": new_agent })))
}


/// Search for a specific environment variable.
///
/// Returns an Option containing its value if the variable exists, otherwise returns None
pub fn get_environment_var(var: &str, env: &Vec<String>) -> Option<String> {
	env.iter().map(|v| v.split('=').collect::<Vec<&str>>()).filter_map(|x| {
		x[0].eq_ignore_ascii_case(var).then(|| x[1..].concat())
	}).next()
}

/// Parses beacon data and returns a Host Record struct 
fn create_host_record(beacon_data: &BeaconData) -> HostRecord {
	let arch = match get_environment_var("PROCESSOR_ARCHITECTURE", &beacon_data.env).unwrap_or("Unknown".to_string()).as_str() {
		"AMD64" => CPUArch::AMD64,
		"ARM64" => CPUArch::ARM64,
		"x86" => CPUArch::X86,
		_ => CPUArch::Unknown
	};

	let mut users: Vec<String> = Vec::new();
	users.push(beacon_data.username.clone());

	HostRecord {
		hostname: beacon_data.hostname.clone(),
		os: OSInfo {
			family: get_environment_var("OS", &beacon_data.env).unwrap_or("Unknown".to_string()),
			version: format!("{0}.{1} Build {2}", beacon_data.version.0, beacon_data.version.1, beacon_data.version.2)
		},
		users,
		arch
	}
}


// TODO CLEANUP THIS FUNCTION
/// Receive beacon data
async fn beacon_handler(
	Extension(auth): Extension<AgentData>,
	State(state): State<Arc<AppState>>,
	Json(beacon_data): Json<BeaconData>,
) -> Result<(), crate::error::Error> {
	state.db.signin(Root { username: &CFG.db.user, password: &CFG.db.pass }).await?;
	state.db.use_ns(&CFG.db.ns).use_db(&CFG.db.db).await?;

	info!({id=auth.record.id.to_string()/*,data=?beacon_data*/}, "Receiving beacon.");

	let sql = "SELECT ->target->host FROM $agent;";
	let mut res = state.db
		.query(sql)
		.bind(("agent", auth.record.id.clone()))
		.await?;
	let host_target: Option<HostTarget> = res.take("->target")?;
	// debug!("{host_id:#?}");

	// Unwrap OK > Always Some, even if there's no results
	if host_target.clone().unwrap().host.is_empty() {
		// Create Host record
		let new_id = uuid::Uuid::new_v4().to_string();
		let _res: Option<HostRecord> = state.db
			.insert(("host", new_id.clone()))
			.content(create_host_record(&beacon_data))
			.await?;

		// Update Agent record
		let agent_rec = auth.record.clone();
		let host_id = Thing::from(("host".to_string(), new_id.clone()));
		let _res: Option<AgentRecord> = state.db
			.update(&agent_rec.id)
			.content(NoIdAgentRecord {
				time: auth.record.time.clone(),
				key: auth.record.key.clone(),
			})
			.await?;
		// debug!("{agent_rec:#?}");

		// Relate Agent to Host
		state.db.query("RELATE $agent_id->target->$host_id;")
			.bind(("agent_id", auth.record.id.clone()))
			.bind(("host_id", host_id.clone()))
			.await?;
	} else {
		let host_id = host_target.unwrap().host.first().unwrap().clone();
		let _res: Option<HostRecord> = state.db
			.update(host_id)
			.content(create_host_record(&beacon_data))
			.await?;
	}

	Ok(())
}