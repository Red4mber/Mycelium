use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

/// Not used yet
/// Describes the access level of an agent on the machine it is on
#[derive(Serialize, Deserialize, Debug)]
pub enum AccessLevel {
	System,
	Administrator,
	User,
	Service
}

/// Describes a row in the `agents` table in the database
#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
	pub id: Uuid,
	pub first_ping: chrono::DateTime<chrono::Utc>,
	pub last_ping: chrono::DateTime<chrono::Utc>,
	pub address: IpNetwork,
	pub operator: Uuid,
	pub notes: Option<String>,
}


/// I didn't manage to add a custom enum type in the postgres db
/// So we're stuck with repr(i32), but I'll make something better later
/// to have more granular control over accounts privileges
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum OperatorRole {
	Admin = 2,
	Operator = 1,
	Guest = 0
}
impl From<i32> for OperatorRole {
	fn from(value: i32) -> Self {
		match value {
			2 => Self::Admin,
			1 => Self::Operator,
			0 => Self::Guest,
			_ => Self::Guest,
		}
	}
}

/// Describes a row in the `operators` table in the database
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

/// Describes an operator account, but without sensitive information such as email or pw hash
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperatorPublicInfo {
	pub id: Uuid,
	pub name: String,
	pub role: OperatorRole,
	pub created_by: Uuid,
	pub created_at: chrono::DateTime<chrono::Utc>,
}


/// Agents need access control too, but I have still yet to figure that out
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType { Agent, Operator }

/// Payload used in the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: Uuid,
	pub iat: usize,
	pub exp: usize,
	pub typ: TokenType,
}

/// Data sent during the operators login process
#[derive(Debug, Deserialize)]
pub struct SignInData {
	pub email: String,
	pub password: String,
}

/// Data needed to create a new operator account
#[derive(Debug, Deserialize)]
pub struct CreateAccountData {
	pub name: String,
	pub email: String,
	pub password: String,
	pub role: OperatorRole,
}