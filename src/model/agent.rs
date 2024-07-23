use serde::{Deserialize, Serialize};



/// Basic host information gathered by the implant,
/// 
/// Everything can be read from the PEB/TEB with no API calls to windows
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BeaconData {
	pub hostname: String,
	pub username: String,
	pub tmpdir: String,
	pub appdata: String,
	pub windir: String,
	pub cwd: String,
	pub cmdline: String,
	pub pid: u64,
	pub env: Vec<String>,
}