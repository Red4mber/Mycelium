mod records;
pub mod auth;


use std::collections::HashMap;
use rsa::RsaPrivateKey;
pub use records::*; // Flatten the modules tree a little bit
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use auth::JwkSet;



#[derive(Debug, Clone)]
pub struct AppState {
	pub db: Surreal<Any>,
	pub jwks: JwkSet,
	pub keys:  HashMap<String, RsaPrivateKey>
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BeaconData {
	pub hostname: String,
	pub username: String,
	pub version: (u32, u32, u32),
	pub tmpdir: String,
	pub appdata: String,
	pub windir: String,
	pub cwd: String,
	pub cmdline: String,
	pub pid: u64,
	pub env: Vec<String>,
}
