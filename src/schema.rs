use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::{Timestamp, Uuid};


#[derive(Serialize, Deserialize, Debug)]
pub enum AccessLevel {
	System,
	Administrator,
	User,
	Service
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
	pub id: Uuid,
	pub first_ping: chrono::DateTime<chrono::Utc>,
	pub last_ping: chrono::DateTime<chrono::Utc>,
	pub address: IpNetwork,
	pub notes: Option<String>,
}

