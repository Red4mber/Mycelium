use std::sync::Arc;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use crate::AppState;
use crate::authentication::agent_middleware;
use crate::model::{BeaconData, CPUArch, HostRecord, OSInfo};

mod beacon;
mod upload;
mod tasks;


/// Returns all the publicly accessible routes
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/poll", get(tasks::task_poll_handler))
		.route("/update_task", post(tasks::task_update_handler))
		.route("/beacon", post(beacon::beacon_handler))
		.route("/upload/:file_name", post(upload::upload_handler))
		.layer(from_fn_with_state(app_state.clone(), agent_middleware))
		.with_state(app_state)
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


