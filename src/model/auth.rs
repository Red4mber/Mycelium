use std::fmt::Display;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::CFG;
use crate::model::OperatorRecord;


#[non_exhaustive]
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Claims {
	/// Issued At Time - Time at which the JWT was issued.
	pub iat: Option<i64>,
	/// Not Before Time - Time before which the JWT must not be accepted for processing.
	pub nbf: Option<i64>,
	/// Expiration Time - Time after which the JWT expires.
	pub exp: Option<i64>,
	/// Issuer - Identifies the principal that issued the JWT.
	pub iss: Option<String>,
	/// Audience - Recipient for which the JWT is intended.
	pub aud: Option<String>,
	/// Subject - Identifies the subject of the JWT (i.e The User).
	pub sub: Option<String>,
	/// JWT ID - Unique identifier for this JWT
	pub jti: Option<String>,
	/// Namespace - The SurrealDB Namespace the token is intended for.
	pub ns: Option<String>,
	/// Database - The Database Namespace the token is intended for.
	pub db: Option<String>,
	/// The Access method this token is intended for.
	pub ac: Option<String>,
	/// The identifier of the record associated with the token.
	pub id: Option<String>,
	/// SurrealDB System user roles (like `Owner` or `Editor`)
	pub rl: Option<Vec<String>>,
}
impl Claims {
	pub fn new(id: String, ac: String, sub: String) -> Self {
		Claims {
			iat: Some(Utc::now().timestamp()),
			nbf: Some(Utc::now().timestamp()),
			exp: Some((Utc::now() + CFG.jwt.ttl).timestamp()),
			jti: Some(Uuid::now_v7().to_string()),
			aud: Some("Mycelium".to_string()),
			sub: Some(sub),
			id:  Some(id),
			iss: Some(CFG.jwt.iss.to_string()),
			ns:  Some(CFG.db.ns.to_string()),
			db:  Some(CFG.db.db.to_string()),
			ac:  Some(ac),
			rl:  None,
		}
	}
}


#[derive(Debug, Clone)]
pub struct AuthData {
	/// Contains all the Claims decoded from the JWT
	pub jwt: Claims,
	/// Contains the database record of the current user
	pub rec: OperatorRecord
}



/// JSON Web Key (JWK)
///
/// Represents a cryptographic key.
/// The fields of the structure represent properties of the key, including its value.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jwk {
	/// The family of cryptographic algorithms used with the key.
	pub kty: String,
	/// Encryption Algorithm
	pub alg: String,
	/// The modulus for the RSA public key.
	pub n: String,
	/// The exponent for the RSA public key.
	pub e: String,
	/// The unique identifier for the key.
	pub kid: String,
	/// Optional field. Identifies the intended use of the public key.
	#[serde(skip_serializing_if = "Option::is_none", rename="use")]
	pub use_: Option<String>,
}

/// JSON Web Key Set (JWKS)
///
/// Represents a set of JWKs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwkSet {
	/// Array of JWKs.
	pub keys: Vec<Jwk>,
}
impl JwkSet {
	pub fn find_key(&self, kid: &str) -> Option<&Jwk>  {
		self.keys.iter().find(|key| key.kid == kid)
	}
}
impl Display for JwkSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self).expect("Failed to serialize JWKS"))
	}
}

