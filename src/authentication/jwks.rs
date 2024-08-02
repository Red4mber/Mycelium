use std::collections::HashMap;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::traits::PublicKeyParts;
use tracing::log::error;
use uuid::Uuid;
use crate::model::auth::{Jwk, JwkSet};
use crate::CFG;



/// Generates a Public/Private key pair.
fn generate_rsa_key(bits: usize) -> (String, RsaPrivateKey) {
	let mut rng = OsRng;
	let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
	let kid = Uuid::new_v4().to_string();
	(kid, private_key)
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

/// Function that initialize the JwkSet with two RSA keys, as example
pub fn prepare_jwkset() -> (JwkSet, HashMap<String, RsaPrivateKey>) {
	let mut priv_keys = HashMap::new();

	if CFG.jwt.persist_keys {
		for entry in glob::glob("./keys/*.pem").unwrap() {              // TODO - De-Unwrap this prototype function
			match entry { 
				Ok(key) => {
					let kid = key.file_name().unwrap().to_str().unwrap().to_string(); // Sheesh that's ugly
					let private_key = RsaPrivateKey::read_pkcs8_pem_file(key).unwrap();
					priv_keys.insert(kid, private_key);
				},
				Err(err) => {
					error!( "Can't read key : {err}");
				}
			}
		}
		if priv_keys.is_empty() {
			let (kid, private_key) = generate_rsa_key(2048);
			private_key.write_pkcs8_pem_file(format!("./keys/{kid}.pem"), LineEnding::default()).expect("TODO");
			priv_keys.insert(kid, private_key);
		}
	} else {
		let (kid, private_key) = generate_rsa_key(2048);
		priv_keys.insert(kid, private_key);
	}
	
	let mut jwks_keys = Vec::new();
	for (kid, privkey) in &priv_keys {
		let pub_key = RsaPublicKey::from(privkey);
		let jwk = rsa_public_key_to_jwk(&pub_key, kid.as_str());
		jwks_keys.push(jwk);
	}
	let jwks = JwkSet { keys: jwks_keys };
	
	(jwks, priv_keys)
}


