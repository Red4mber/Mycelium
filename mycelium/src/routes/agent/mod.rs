use std::sync::Arc;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use crate::AppState;
use crate::authentication::agent_middleware;

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


