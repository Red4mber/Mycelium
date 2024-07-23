

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod db;



#[derive(Serialize, Deserialize, Debug)]
pub struct BeaconData {
    pub data: String
}



/// Stores the keys used to encode and decodes JWTs
#[derive(Clone)]
pub struct AuthKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}




/// I didn't manage to add a custom enum type in the postgres db
/// So we're stuck with repr(i32), but I'll make something better later
/// to have more granular control over accounts privileges
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum OperatorRole {
    Admin = 2,
    Operator = 1,
    Guest = 0,
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



/// Describes an operator account, but without sensitive information such as email or pw hash
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperatorPublicInfo {
    pub id: Uuid,
    pub name: String,
    pub role: OperatorRole,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
// 
// /// Agents need access control too, but I have still yet to figure that out
// #[derive(Debug, Serialize, Deserialize)]
// pub enum TokenType {
//     Agent,
//     Operator,
// }

/// Payload used in the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
    // pub typ: TokenType,
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
