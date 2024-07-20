use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
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
	pub id: Uuid,
	pub first_ping: chrono::DateTime<chrono::Utc>,
	pub last_ping: chrono::DateTime<chrono::Utc>,
	pub address: IpNetwork,
	pub notes: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[repr(i32)]
pub enum OperatorRole {
	Admin = 0,
	Operator = 1,
	Guest = 2
}

impl From<i32> for OperatorRole {
	fn from(value: i32) -> Self {
		match value {
			0 => Self::Admin,
			1 => Self::Operator,
			2 => Self::Guest,
			_ => Self::Guest,
		}
	}
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Operator {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub password: String,
	pub role: OperatorRole,
	pub created_by: Uuid,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub last_login: chrono::DateTime<chrono::Utc>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperatorPublicInfo {
	pub id: Uuid,
	pub name: String,
	pub role: OperatorRole,
	pub created_by: Uuid,
	pub created_at: chrono::DateTime<chrono::Utc>,
}



#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType { Agent, Operator }

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: Uuid,
	pub iat: usize,
	pub exp: usize,
	pub typ: TokenType,
}

#[derive(Debug, Deserialize)]
pub struct OperatorRegisterData {
	pub referer: Uuid,
	pub name: String,
	pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct OperatorSignInData {
	pub email: String,
	pub password: String,
}