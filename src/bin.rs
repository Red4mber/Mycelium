use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt
};
use surrealdb::{
    engine::any,
    engine::any::Any,
    opt::auth::Root
};
use tracing::info;

pub use mycelium::*;




#[tokio::main]
async fn main() -> Result<(), Error> {
    info!("Mycelium server is starting....");
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or(CFG.trace.env_filter.parse().unwrap()),
        ).with(tracing_subscriber::fmt::layer()).init();

    let db_client = any::connect(&CFG.db.conn).await.map_err(Error::DatabaseError)?;

    // Sign in as root
    db_client.signin(Root {
        username: &CFG.db.user,
        password: &CFG.db.pass,
    }).await.map_err(Error::DatabaseError)?;
    
    // Prepare the state of the application
    let (jwks, private_keys) = authentication::jwks::generate_jwkset().unwrap();
    let app_state = Arc::new(AppState { db: db_client, jwks });
    
    // Fetches routes from our different routers and merges them in a single one
    let app = Router::new()
        .merge(routes::public::get_routes(app_state.clone()))
        .nest("/user",  routes::operator::get_routes(app_state.clone()))
        .nest("/agent", routes::agent::get_routes(app_state.clone()))
        .nest("/host",  routes::host::get_routes(app_state.clone()))
        .nest("/file",  routes::file::get_routes(app_state.clone()))
        .nest("/auth",  routes::auth::get_routes(app_state.clone()))
        .with_state(app_state.clone());

    // run our app
    let listener = tokio::net::TcpListener::bind(
        CFG.http.listener.str()
    ).await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    Ok(())
}
