use std::fmt::Display;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use rsa::traits::PublicKeyParts;


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
	/// Tan array of JWKs.
	pub keys: Vec<Jwk>,
}

impl Display for JwkSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self).expect("Failed to serialize JWKS"))
	}
}

/// Generates a Public/Private key pair.
fn generate_rsa_key_pair(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
	let mut rng = OsRng;
	let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
	let public_key = RsaPublicKey::from(&private_key);
	(private_key, public_key)
}



fn rsa_public_key_to_jwk(public_key: &RsaPublicKey, kid: &str) -> Jwk {
	let n = general_purpose::URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be());
	let e = general_purpose::URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be());

	Jwk {
		kty: "RSA".to_string(),
		n,
		e,
		alg: "RS256".to_string(),
		kid: kid.to_string(),
		use_: Some("sig".to_string()),
	}
}

pub fn generate_jwkset() -> (JwkSet, Vec<RsaPrivateKey>) {
	let (private_key1, public_key1) = generate_rsa_key_pair(2048);
	let (private_key2, public_key2) = generate_rsa_key_pair(2048);

	let jwk1 = rsa_public_key_to_jwk(&public_key1, "key1");
	let jwk2 = rsa_public_key_to_jwk(&public_key2, "key2");

	let jwks = JwkSet { keys: vec![jwk1, jwk2] };
	let priv_keys = vec![private_key1, private_key2];
	(jwks, priv_keys)
}


