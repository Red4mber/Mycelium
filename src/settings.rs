//
// This big wall of structs represents the `config.toml` file found at the root of the crate
// It's all made available globally by the `SETTINGS` mutex.
//
use chrono::Duration;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new("settings.toml");
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub tracing: Tracing,
    pub database: Database,
    pub http: Http,
    pub tokens: Tokens,
}

impl Settings {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(path)
            .expect("config file should be present in the root of the crate.\n");
        let settings: Settings = toml::from_str(&content).expect(
            "config file should be properly formatted. See `settings.example.toml` for an example.\n",
        );
        settings
    }
    pub fn _save_to_file<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(&self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tracing {
    pub env_filter: String,
}

/// Settings related to the Postgresql Database
#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    host: String,
    username: String,
    password: String,
    db_name: String,
}
impl Database {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.host, self.db_name
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub listener: Listener,
    pub routes: Routes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Listener {
    pub addr: String,
    pub port: u16,
}

/// Parent structure containing all the routes
#[derive(Debug, Deserialize, Serialize)]
pub struct Routes {
    pub unauthenticated: UnauthenticatedRoutes,
    pub operators: AuthenticatedRoutes,
    pub implant: ImplantRoutes,
}

/// The login route and the debug routes
#[derive(Debug, Deserialize, Serialize)]
pub struct UnauthenticatedRoutes {
    pub login: String,
    pub ping: String,
    pub healthcheck: String,
}

/// All these routes that are accessible only after login
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedRoutes {
    pub lookup_operator: String,
    pub all_operators: String,
    pub new_operator: String,
    pub who_am_i: String,
    pub lookup_agent: String,
    pub all_agents: String,
}

/// All these routes are meant for implants communicating with the server via HTTP
#[derive(Debug, Deserialize, Serialize)]
pub struct ImplantRoutes {
    pub beacon: String,
}


/// Parent structure containing settings related to the JWT
#[derive(Debug, Deserialize, Serialize)]
pub struct Tokens {
    /// Contains the time to live of the auth tokens
    #[serde(
        deserialize_with = "deserialize_duration",
        serialize_with = "serialize_duration"
    )]
    pub ttl: Duration
}



/// Custom deserializer for TTL duration
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    parse_time_delta(&s).map_err(serde::de::Error::custom)
}

/// Custom Serialization function for TTL duration
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

// Same as above, just the other way around
// i don't even know why i write these myself...
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
    parts.join("")
}
