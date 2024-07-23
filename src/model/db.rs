
// These structures all describe tables in the database

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use tokio_postgres::types::{FromSql, ToSql};
use uuid::Uuid;

use crate::model::OperatorPublicInfo;

/// Describes a row in the `agents` table in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
	pub id: Uuid,
	pub first_ping: chrono::DateTime<chrono::Utc>,
	pub last_ping: chrono::DateTime<chrono::Utc>,
	pub host_id: Uuid,
	pub operator_id: Uuid,
	pub notes: Option<String>,
}
impl From<Row> for Agent {
	fn from(row: Row) -> Self {
		Self {
			id: row.get("id"),
			first_ping: row.get("first_ping"),
			last_ping: row.get("last_ping"),
			host_id: row.get("host_id"),
			operator_id: row.get("operator_id"),
			notes: row.get("notes")
		}
	}
}


/// Describes the different roles an Operator account can be.
#[derive(FromSql, ToSql, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[postgres(name = "operator_role")]
pub enum OperatorRole {
	#[postgres(name = "admin")]
	Admin,
	#[postgres(name = "operator")]
	Operator,
	#[postgres(name = "guest")]
	Guest,
}
/// Describes a row in the `operators` table in the database
#[derive(Deserialize, Serialize, Clone, Debug)]
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

impl Operator {
	pub fn public_info(self) -> OperatorPublicInfo {
		OperatorPublicInfo {
			id: self.id,
			name: self.name,
			role: self.role,
			created_by: self.created_by,
			created_at: self.created_at,
		}
	}
}
impl From<Row> for Operator {
	fn from(row: Row) -> Self {
		Self {
			id: row.get("id"),
			name: row.get("name"),
			email: row.get("email"),
			password: row.get("password"),
			role: row.get("role"),
			created_by: row.get("created_by"),
			created_at: row.get("created_at"),
			last_login: row.get("last_login")
		}
	}
}

