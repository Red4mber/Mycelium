//
// This big wall of structs represents the `config.toml` file found at the root of the crate
// It's all made available globally by the `SETTINGS` mutex.
//
use chrono::Duration;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{ collections::HashMap, fs, path::Path, sync::LazyLock };


// Hell Yeah - LazyLock Stable after Rust 1.80.0  \o/
pub static CFG: LazyLock<Settings> = LazyLock::new(|| {
    Settings::new("Settings.toml")
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(rename = "tracing")]
    pub trace: Tracing,
    #[serde(rename = "database")]
    pub db: DbParameters,
    pub http: Http,
    #[serde(rename = "tokens")]
    pub jwt: Tokens,
    #[serde(rename = "misc")]
    pub misc: Misc,
}
impl Settings {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(path)
            .expect("config file should exist\n");
        let settings: Settings = toml::from_str(&content)
            .expect("config file should be properly formatted.\n" );
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

/// Settings related to the database connection
#[derive(Debug, Deserialize, Serialize)]
pub struct DbParameters {
    #[serde(rename = "connection")]
    pub conn: String,
    #[serde(rename = "username")]
    pub user: String,
    #[serde(rename = "password")]
    pub pass: String,
    #[serde(rename = "database")]
    pub db: String,
    #[serde(rename = "namespace")]
    pub ns: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub listener: Listener,
    // pub routes: Routes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Listener {
    #[serde(rename = "address")]
    pub addr: String,
    #[serde(rename = "port")]
    pub port: u16,
}
impl Listener {
    pub fn str(&self) -> String { 
        format!("{}:{}",self.addr,self.port) 
    }
}



/// Parent structure containing settings related to the JWT
#[derive(Debug, Deserialize, Serialize)]
pub struct Tokens {
    /// Contains the time to live of the auth tokens
    #[serde(
        deserialize_with = "deserialize_duration",
        serialize_with = "serialize_duration"
    )]
    pub ttl: Duration,
    pub iss: String,
    pub persist_keys: bool,
    pub key_dir: String
}

/// All the settings that didn't fit anywhere else
#[derive(Debug, Deserialize, Serialize)]
pub struct Misc {
    pub uploads_dir: String,
}

// Above this line are the structs representing Mycelium's settings
// Underneath here are the utility functions used to parse settings

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

/// Utility function to parse a string like (`05d 08h 04m 02s`)
/// into a TimeDelta (alias of Duration)
pub fn parse_time_delta(input: &str) -> Result<Duration, String> {
    let re = Regex::new(r"(\d+)\s*([a-zA-Z]+)").unwrap(); // regexr.com/83oo7
    let unit_map = HashMap::from([
        ("s", 1), ("seconds", 1),
        ("m", 60), ("minutes", 60),
        ("h", 3600), ("hours", 3600),
        ("d", 86400), ("days", 86400),
        ("w", 604800), ("weeks", 604800),
    ]);

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

/// Same as above, just the other way around
/// i don't even know why i write these myself...
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let days    =  total_seconds / 86400;
    let hours   = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds =  total_seconds % 60;

    let mut parts = Vec::new();
    if days > 0                             { parts.push(format!("{:02}d", days));    }
    if hours > 0    || !parts.is_empty()    { parts.push(format!("{:02}h", hours));   }
    if minutes > 0  || !parts.is_empty()    { parts.push(format!("{:02}m", minutes)); }
    parts.push(format!("{:02}s", seconds));
    parts.join("")
}
