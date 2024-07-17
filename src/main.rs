
use std::sync::Arc;
use std::time::Duration;
use axum::{
    Json,
    Router,
    async_trait,
    routing::get,
    routing::post,
    http::StatusCode,
    http::request::Parts,
    response::IntoResponse,
    extract::{FromRef, FromRequestParts},
};
use sqlx::{Executor, PgPool, Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::error::internal_error;
use crate::agent::lookup_agent;

mod agent;
mod error;
mod schema;



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
            tracing::debug!("Successfully connected to the database");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to the postgres database");
            std::process::exit(-1)
        }
    };

    // // RESETS THE DATABASE
    // // Remove before production
    // db_pool.execute(include_str!("../migrations/schema.sql"))
    //     .await
    //     .context("Failed to initialize DB")?;


    // create routes
    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        // .route("/api/agent", get(list_agents))
        .route("/api/agent/:id", get(lookup_agent))
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
