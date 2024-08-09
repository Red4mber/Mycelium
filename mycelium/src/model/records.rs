use serde::{Deserialize, Serialize, Serializer};
use surrealdb::sql::{Datetime, Thing};
use crate::model::BeaconData;


/// Struct used when querying a "Target" graph to see if an implant is on a Host
#[derive(Serialize, Deserialize, Clone)]
pub struct HostTarget {
	#[serde(rename = "->host")]
	pub host: Vec<Thing>
}

/// Struct used when querying a "Execute" graph to query an agent's tasks
#[derive(Serialize, Deserialize, Clone)]
pub struct AgentTasks {
	#[serde(rename = "->task")]
	pub task: Vec<Thing>
}


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
/// Represents a row of the `Host` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostRecord {
	pub arch: CPUArch,
	pub hostname: String,
	pub users: Vec<String>,
	pub os: OSInfo
}
impl From<BeaconData> for HostRecord {
	fn from(data: BeaconData) -> Self {
		let arch = match data.arch.as_str() {
			"AMD64" => CPUArch::AMD64,
			"ARM64" => CPUArch::ARM64,
			"x86" => CPUArch::X86,
			_ => CPUArch::Unknown
		};
		Self {
			arch,
			hostname: data.hostname,
			users: data.users,
			os: OSInfo { family: data.os_family, version: data.os_version },
		}
	}
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

/// Enum representing the status of a Task
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
	Pending, Running, Error, Success
}

/// Represents a row of the `Task` table
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskRecord {
	#[serde(serialize_with = "simple_serializer")]
	pub id: Thing,
	pub time: TimeRecord,
	pub command: String,
	pub output: String,
	pub status: TaskStatus,
}


fn simple_serializer<T,S>(thing: T, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer, T: ToString {
	serializer.serialize_str(thing.to_string().as_str())
}
