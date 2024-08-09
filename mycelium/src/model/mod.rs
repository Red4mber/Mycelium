mod records;
pub mod auth;


use std::collections::HashMap;
use rsa::RsaPrivateKey;
pub use records::*; // Flatten the modules tree a little bit
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use auth::JwkSet;


/// Data shared with every middlewares and route handlers
/// 
/// Contains the keys for signing and verifying user tokens
/// and the connection handler for the SurrealDB database
#[derive(Debug, Clone)]
pub struct AppState {
	pub db: Surreal<Any>,
	pub jwks: JwkSet,
	pub keys:  HashMap<String, RsaPrivateKey>
}


/// Represents the data sent from an agent when beaconing \
/// It contains all the minimum necessary info to register a new host in the database
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BeaconData {
	pub hostname: String,
	pub users: Vec<String>,
	pub os_family: String,
	pub os_version: String,
	pub arch: String,
}
