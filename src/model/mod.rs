

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::db::OperatorRole;


pub mod db;
pub mod agent;


/// Stores the keys used to encode and decodes JWTs inside the AppState
#[derive(Clone)]
pub struct AuthKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
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

/// Payload used in the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
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
