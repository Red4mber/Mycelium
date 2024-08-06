use serde::{Deserialize, Serialize, Serializer};
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
	#[serde(serialize_with = "simple_serializer")]
	pub id: Thing,
	#[serde(skip_serializing_if = "Option::is_none",serialize_with = "opt_serializer")]
	pub host: Option<Thing>,
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
	#[serde(serialize_with = "datetime_serializer")]
	pub created_at: Datetime,
	#[serde(serialize_with = "datetime_serializer")]
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
fn opt_serializer<T,S>(thing: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer, T: ToString, T: Clone {
	// TODO Kinda ugly - Can we do without clone ?
	serializer.serialize_str(thing.clone().unwrap().to_string().as_str())
}

fn datetime_serializer<S>(datetime: &Datetime, serializer: S) -> Result<S::Ok, S::Error> 
where S: Serializer {
	serializer.serialize_str(
		datetime
			.format("%d/%m/%Y %H:%M")
			.to_string()
			.as_str())
}
