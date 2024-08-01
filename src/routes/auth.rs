use std::sync::Arc;
use axum::extract::State;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::EncodePrivateKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::opt::auth::Root;
use crate::{AppState, CFG, Error, model::auth::Claims};
use crate::model::OperatorRecord;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginData {
	email: String,
	password: String,
}


pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/jwks", get(jwks_handler))
		.route("/login", post(login_handler))
		.with_state(app_state)
}

pub async fn jwks_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
	Json(&state.jwks).into_response()
}

#[axum_macros::debug_handler]
pub async fn login_handler(
	State(state): State<Arc<AppState>>,
	Json(login_data): Json<LoginData>,
) -> Result<Json<Value>, Error> {
	state.db.signin(Root {
		username: &CFG.db.user,
		password: &CFG.db.pass,
	}).await?;
	state.db.use_ns(&CFG.db.ns)
	     .use_db(&CFG.db.db)
	     .await?;
	
	let mut response = state.db
		.query("SELECT * FROM operator WHERE email = $email AND crypto::argon2::compare(pass, $pass)")
		.bind(("email", login_data.email))
		.bind(("pass", login_data.password)).await?;
	let res: Option<OperatorRecord> = response.take(0)?;
	let claims = match res {
		None => return Err(Error::WrongCredentials),
		Some(operator) => {
			Claims::new(operator.id.to_string(), "operator".to_string(), operator.name)
		}
	};
	let mut header = Header::new(Algorithm::RS256);
	
	// Get any key
	let (kid, private_key) = state.keys
		.iter()
		.next()
		.unwrap();

	header.kid = Some(kid.to_string());

	let enc_key = EncodingKey::from_rsa_pem(
		private_key
			.to_pkcs8_pem(LineEnding::default())
			.unwrap()
			.as_bytes()
	).unwrap();
	
	let token = encode(&header, &claims, &enc_key).unwrap();
	
	Ok(Json(serde_json::json!({
        "token": token,
    })))
}