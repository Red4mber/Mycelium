use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use crate::model::{CPUArch, OSInfo};


/// Represents a row of the `Host` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostRecord {
	arch: CPUArch,
	hostname: String,
	os: OSInfo
}
/// Represents a row of the `Agent` table 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentRecord {
	pub id: Thing,
	pub time: TimeRecord
}

/// Represents a row of the `File` table 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileRecord {
	pub id: Thing,
	pub time: TimeRecord
}

/// Represents the time object found in `Agent` and `File` 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRecord {
	pub created_at: Datetime,
	pub updated_at: Datetime,
}

/// Represents a row of the `Operator` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperatorRecord {
	pub id: Thing,
	pub name: String,
	pub email: String,
	pub admin: bool,

}




