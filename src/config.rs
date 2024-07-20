use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::Duration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use regex::Regex;

//
// Big wall of structs :D
// represents the `config.toml` file found at the root of the crate
//

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
	pub tracing: Tracing,
	pub database: Database,
	pub http: Http,
	pub tokens: Tokens,
}

impl Settings {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
		let content = fs::read_to_string(path)?;
		let settings: Settings = toml::from_str(&content)?;
		Ok(settings)
	}
	pub fn _save_to_file<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
		let content = toml::to_string(&self)?;
		fs::write(path, content)?;
		Ok(())
	}

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tracing {
	pub env_filter: String
}

/// Settings related to the Postgresql Database
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
	host: String,
	username: String,
	password: String,
	db_name: String,
}
impl Database {
	pub fn get_uri(&self)-> String {
		format!("postgres://{}:{}@{}/{}", self.username, self.password, self.host, self.db_name)
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Http {
	pub listener: Listener,
	pub routes: Routes,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Listener {
	pub addr: String,
	pub port: u16,
}

/// Parent structure containing all the routes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Routes {
	pub unauthenticated: UnauthenticatedRoutes,
	pub operators: OperatorRoutes,
	pub agents: AgentRoutes,
}

/// The login route and the debug routes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnauthenticatedRoutes {
	pub login: String,
	pub ping: String,
	pub healthcheck: String,
}

/// All routes to Operators API endpoints
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperatorRoutes {
	pub lookup: String,
	pub all: String,
	pub new: String,
	pub me: String,
}

/// Technically they're operator routes as well \
/// sorry i lied :c
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentRoutes {
	pub lookup: String,
	pub all: String,
	pub new: String,
	pub me: String,
}

/// Parent structure containing settings related to the JWT
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tokens {
	pub ttl: Ttl,
	// pub key_path: Box<Path>,         // Spent like 8 hours trying to (De)Serialize these keys  
	// pub keys: Arc<RefCell<Keys>>,    // That's a night i will never recover 
	// pub regenerate_keys: bool,       // i want to sleeeep
}

/// Contains the time to live of the tokens depending on their type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ttl {
	#[serde(
		deserialize_with = "deserialize_duration",
		serialize_with = "serialize_duration"
	)]
	pub operators: Duration,
	#[serde(
		deserialize_with = "deserialize_duration",
		serialize_with = "serialize_duration"
	)]
	pub agents: Duration,
}

// Custom deserializer for TTL duration
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	parse_time_delta(&s).map_err(serde::de::Error::custom)
}

// Same but the other way
pub fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let formatted = format_duration(*duration);
	serializer.serialize_str(&formatted)
}


// Utility function to parse a string like (`05d 08h 04m 02s`)
// into a TimeDelta (alias of Duration)
pub fn parse_time_delta(input: &str) -> Result<Duration, String> {
	let unit_map = HashMap::from([
		("s", 1),
		("seconds", 1),
		("m", 60),
		("minutes", 60),
		("h", 3600),
		("hours", 3600),
		("d", 86400),
		("days", 86400),
		("w", 604800),
		("weeks", 604800),
	]);
	// regex tested and should work >> regexr.com/83oo7
	let re = Regex::new(r"(\d+)\s*([a-zA-Z]+)").unwrap();

	let mut total_seconds = 0;
	for cap in re.captures_iter(input) {
		let value: i64 = cap[1].parse().map_err(|_| "Invalid number format")?;
		let unit = cap[2].to_lowercase();
		if let Some(&multiplier) = unit_map.get(unit.as_str()) {
			total_seconds += value * multiplier;
		}
	}
	Ok(Duration::seconds(total_seconds))
}

// Guess what ? The exact opposite ! :D
pub fn format_duration(duration: Duration) -> String {
	let total_seconds = duration.num_seconds();
	let days = total_seconds / 86400;
	let hours = (total_seconds % 86400) / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let seconds = total_seconds % 60;

	let mut parts = Vec::new();
	if days > 0 {
		parts.push(format!("{:02}d", days));
	}
	if hours > 0 || !parts.is_empty() {
		parts.push(format!("{:02}h", hours));
	}
	if minutes > 0 || !parts.is_empty() {
		parts.push(format!("{:02}m", minutes));
	}
	parts.push(format!("{:02}s", seconds));
	parts.join(" ")
}