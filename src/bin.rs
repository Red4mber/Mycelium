use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::SETTINGS;
use crate::auth::generate_encryption_keys;
use crate::model::AuthKeys;


mod auth;
mod error;
mod model;
mod routes;
mod settings;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    operator_keys: AuthKeys,
}

// // Utility function that returns a slice containing the raw bytes of any `Sized` type
unsafe fn _any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts((p as *const T) as *const u8, core::mem::size_of::<T>())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Set-up tracing subscriber, using the environment or a default
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or(SETTINGS.tracing.env_filter.parse().unwrap()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                // .pretty()
        )
        .init();

    // Database connect
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(SETTINGS.database.url().as_str())
        .await
    {
        Ok(pool) => {
            tracing::info!("Successfully connected to the database!");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to the postgres database: {e}");
            std::process::exit(-1)
        }
    };

    let (encoding_key, decoding_key) = generate_encryption_keys();
    let state = Arc::new(AppState {
        db: db_pool.clone(),
        operator_keys: AuthKeys {
            encoding_key, decoding_key
        },
    });
    
    let app = Router::new()
        .merge(routes::public::get_routes(Arc::clone(&state)))
        .merge(routes::authenticated::get_routes(Arc::clone(&state)))
        .merge(routes::implants::get_routes(Arc::clone(&state)))
        .with_state(state);

    // run our app
    let addr = format!(
        "{}:{}",
        SETTINGS.http.listener.addr, SETTINGS.http.listener.port
    );
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
// Made it a service that extract SocketAddr so I can log the client's IP if anything fishy occurs 
}
