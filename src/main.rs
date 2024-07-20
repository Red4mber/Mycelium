use std::sync::Arc;
use std::time::Duration;

use axum::{body::Bytes, Json, middleware, response::IntoResponse, Router, routing::{get, post}};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::{
	layer::SubscriberExt,
	util::SubscriberInitExt,
};

use crate::{
	auth::{auth, generate_encryption_keys},
	routes::agent::{list_all_agents, lookup_agent_by_id},
	routes::operator::{create_operator_account, list_all_operators, lookup_operator_by_id, operator_login, show_current_operator},
};
use crate::config::Settings;


mod error;
mod model;
mod routes;
mod auth;
mod config;


#[derive(Clone)]
pub struct AppState {
	db: PgPool,
	keys: Keys,
	cfg: Settings,
}


// Stores the actual keys, after having decoded them from base64
#[derive(Clone)]
pub struct Keys {
	pub encoding_key: EncodingKey,
	pub decoding_key: DecodingKey
}


// // Utility function that returns a slice containing the raw bytes of any `Sized` type
unsafe fn _any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
	core::slice::from_raw_parts((p as *const T) as *const u8, core::mem::size_of::<T>())
}


#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let cfg = Settings::from_file("config.toml").unwrap_or_else(|e| {
		eprintln!("Failed to read `config.toml`: {e}");
		std::process::exit(-10)
	});

	// Set-up tracing subscriber, using the environment or a default
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| cfg.clone().tracing.env_filter.into()),
		)
		.with(tracing_subscriber::fmt::layer())
		.init();

	// Database connect
	let db_pool = match PgPoolOptions::new()
		.max_connections(5)
		.acquire_timeout(Duration::from_secs(3))
		.connect(&cfg.database.get_uri())
		.await {
		Ok(pool) => {
			tracing::debug!("Successfully connected to the database!");
			pool
		}
		Err(e) => {
			tracing::error!("Failed to connect to the postgres database: {e}");
			std::process::exit(-1)
		}
	};

	let (encoding_key, decoding_key) = generate_encryption_keys();
	// Generate a structure containing our shared state: a database connexion and a JWT key-pair
	let state = Arc::new(AppState {
		db: db_pool.clone(),
		cfg: cfg.clone(),
		keys: Keys {
			encoding_key,
			decoding_key,
		},
	});
	
	
	let r = & cfg.http.routes;
	// create routes
	let app = Router::new()
		.route( & r.agents.lookup, get(lookup_agent_by_id))
		.route( & r.agents.all, get(list_all_agents))
		.route( & r.operators.all, get(list_all_operators))
		.route( & r.operators.new, post(create_operator_account))
		.route( & r.operators.me, get(show_current_operator))
		.route( & r.operators.lookup, get(lookup_operator_by_id))
			// ^ All routes above this are authenticated ^ //
			.layer(middleware::from_fn_with_state(state.clone(), auth))
		.route( & r.unauthenticated.login, post(operator_login))
		.route( & r.unauthenticated.healthcheck, get(health_check_handler))
		.route( & r.unauthenticated.ping, post(ping_handler))
			.with_state(state);
	
	// run our app
	let addr = format!("{}:{}", &cfg.http.listener.addr, &cfg.http.listener.port);
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	
	tracing::debug!("listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}



// Simple health check endpoint
pub async fn health_check_handler() -> impl IntoResponse {
	Json(serde_json::json!({
        "status": "ok",
    }))
}


#[derive(Deserialize, Serialize)]
pub struct PingParams {
	pub data: String
}


// Simple Ping endpoint
pub async fn ping_handler(
	body: Bytes
) -> impl IntoResponse {
	body
}

