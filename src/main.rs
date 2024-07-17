use std::sync::Arc;
use std::time::Duration;

use axum::{
    Json,
    Router,
    routing::get,
};
use axum::response::IntoResponse;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::agent::{list_all_agents, lookup_agent_by_id};
use crate::operator::{list_all_operators, lookup_operator_by_id};


mod agent;
mod error;
mod model;
mod operator;


pub struct AppState {
    db: PgPool,
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

    // // RESETS THE DATABASE
    // // Remove before shipping
    // db_pool.execute(include_str!("../migrations/20240717103619_initialize_db.sql"))
    //     .await
    //     .context("Failed to initialize DB")?;


    // create routes
    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        // .route("/api/agent", get(list_agents))
        .route("/api/agent/:id", get(lookup_agent_by_id))
        .route("/api/agent/all", get(list_all_agents))
        .route("/api/operator/all", get(list_all_operators))
        .route("/api/operator/:id", get(lookup_operator_by_id))
        // .route("/api/agent", post(register_new_agent))
        // .route("/api/operator", post(register_new_operator))
        .with_state(Arc::new(AppState { db: db_pool.clone() }));

    // run our app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



// Simple health check endpoint
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Mycelium API";
    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });
    Json(json_response)
}
