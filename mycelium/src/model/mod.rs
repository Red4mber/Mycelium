mod records;
pub mod auth;


use std::collections::HashMap;
use rsa::RsaPrivateKey;
pub use records::*; // Flatten the modules tree a little bit
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use auth::JwkSet;


/// Enum representing the most common CPU Architectures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CPUArch {
	X86, AMD64, ARM, ARM64, MIPS, PowerPC, Unknown
}

/// Structure containing Operating system identification information 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OSInfo {
	pub(crate) family: String,
	pub(crate) version: String
}


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
