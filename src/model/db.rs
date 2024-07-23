
// These structures all describe tables in the database

use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;


use crate::model::{OperatorPublicInfo, OperatorRole};


/// Describes a row in the `agents` table in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
	pub id: Uuid,
	pub first_ping: chrono::DateTime<chrono::Utc>,
	pub last_ping: chrono::DateTime<chrono::Utc>,
	pub address: IpNetwork,
	pub operator: Uuid,
	pub notes: Option<String>,
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