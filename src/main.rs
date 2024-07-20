use std::sync::Arc;
use std::time::Duration;

use axum::{Json, Router, routing::{get, post}, response::IntoResponse, middleware};
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


use crate::{
    routes::agent::{list_all_agents, lookup_agent_by_id},
    routes::operator::{list_all_operators, lookup_operator_by_id, operator_login, show_current_operator},
    auth::{generate_encryption_keys, auth},
};


mod error;
mod model;
mod routes;
mod auth;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}



#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set-up tracing subscriber, using the environment or a default
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mycelium=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connect
    let conn_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&conn_url)
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

    // Generate a structure containing our shared state: a database connexion and a JWT key-pair
    let (encoding_key, decoding_key) = generate_encryption_keys();
    let state = Arc::new(AppState {
        db: db_pool.clone(),
        encoding_key,
        decoding_key,
    });

    // create routes
    let app = Router::new()
        // .route("/api/agent", get(list_agents))
        // .route("/api/operator", post(register_new_operator))
        .route("/agent/:id", get(lookup_agent_by_id))
        .route("/agent/all", get(list_all_agents))
        .route("/operator/all", get(list_all_operators))
        .route("/operator", get(show_current_operator))
        .route("/operator/:id", get(lookup_operator_by_id))
        .layer(middleware::from_fn_with_state(state.clone(), auth)) // All routes above are authenticated
        .route("/login", post(operator_login))
        .route("/healthcheck", get(health_check_handler).post(ping_handler))
        // .layer(HandleErrorLayer::new(handle_parsing_error))
        .with_state(state);

    // run our app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
    Json(payload): Json<PingParams>
) -> impl IntoResponse {
    Json(payload)
}

