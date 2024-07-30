mod records;

pub use records::*; // Flatten the modules tree a little bit
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::authentication::jwks::JwkSet;


/// Enum representing the most common CPU Architectures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CPUArch {
	I386, AMD64, ARM, ARM64, MIPS, PowerPC, Unknown
}

/// Structure containing Operating system identification information 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OSInfo {
	family: String,
	version: String
}


#[derive(Debug, Clone)]
pub struct AppState {
	pub db: Surreal<Any>,
	pub jwks: JwkSet,
}
