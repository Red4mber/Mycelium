use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub enum AccessLevel {
	System,
	Administrator,
	User,
	Service
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
	uuid: Uuid,
	ip_addr: IpAddr,
	notes: Option<String>,
	access_level: AccessLevel
}

