use serde::{Deserialize, Serialize, Serializer};
use surrealdb::sql::{Datetime, Thing};
use crate::model::{CPUArch, OSInfo};



/// Struct used when querying a "Target" graph to see if an implant is on a Host
#[derive(Serialize, Deserialize, Clone)]
pub struct HostTarget {
	#[serde(rename = "->host")]
	pub host: Vec<Thing>
}


/// Represents a row of the `Host` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostRecord {
	pub arch: CPUArch,
	pub hostname: String,
	pub users: Vec<String>,
	pub os: OSInfo
}
/// Represents a row of the `Agent` table 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentRecord {
	#[serde(serialize_with = "simple_serializer")]
	pub id: Thing,
	pub time: TimeRecord,
	pub key: String,
}

/// Dirty fix for a bug
/// Can't have ID field when updating a specific record
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoIdAgentRecord {
	pub time: TimeRecord,
	pub key: String,
}

/// Represents a row of the `File` table 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileRecord {
	#[serde(serialize_with = "simple_serializer")]
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
	#[serde(serialize_with = "simple_serializer")]
	pub id: Thing,
	pub name: String,
	pub email: String,
	pub admin: bool,

}


fn simple_serializer<T,S>(thing: T, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer, T: ToString {
	serializer.serialize_str(thing.to_string().as_str())
}
