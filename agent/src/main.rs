use reqwest::blocking::Client;
use reqwest::Error;
use mycelium::model::BeaconData;



fn main() -> Result<(), Error> {
	
	let data = BeaconData {
		data: "test test 123".to_string()
	};
	let http = Client::new();
	let host = "http://localhost:3000/beacon";
	let res = http
		.post(host)
		// .header("Authorization", "51d10216-2daf-41eb-a9ca-a8da3a3cc924") // Test UUID added manually in the database
		.json(&data)
		.send()?;
	println!("{:?}",res.text());
	Ok(())
}