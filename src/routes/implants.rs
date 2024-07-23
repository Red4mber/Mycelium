use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use axum::{http, Json, middleware, Router};
use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;


use crate::model::{BeaconData, db};
use crate::AppState;
use crate::error::Error;
use crate::settings::SETTINGS;


// TODO => PICK A SIDE - Is it Implants or Agents ?


pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	let r = &SETTINGS.http.routes;
	Router::new()
		.route(&r.implant.beacon, post(beacon))

		.layer(middleware::from_fn_with_state(app_state.clone(), implant_middleware))
		.with_state(app_state)
}

pub async fn beacon(
	// Extension(op): Extension<Operator>,
	Json(beacon_data): Json<BeaconData>
) -> Result<impl IntoResponse, Error> {
	tracing::debug!("An implant just connected to the server !");
	tracing::debug!("\n{beacon_data:#?}");
	Ok(Json(json!({})))
}



pub async fn implant_middleware(
	State(state): State<Arc<AppState>>,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	mut req: Request,
	next: Next,
) -> Result<Response, Error> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header = match auth_header {
		Some(val) => val.to_str().map_err(|_| log_err(addr))?,
		None => return Err(log_err(addr)),
	};
	let mut header = auth_header.split_whitespace();
	let str_uuid = header.next().ok_or_else(|| log_err(addr))?;
	let uuid = Uuid::from_str(str_uuid).map_err(|_| log_err(addr))?;

	let implant_data = query_implant_by_id(&uuid, &state.db).await;

	//  TODO Find a better way to authenticate implants
	//
	//  Right now implants can be impersonated if you just steal their UUID
	//  An fun idea :
	//      - [on server] Operator register new implant keys => generates shared secret
	//      - Include shared secret in implants - The other in database with implant uuid as "index"
	//      - [on implant] maybe hkdf or something - encrypt w/ AEAD like chachapoly or something like that
	//      - Asymmetric & Unique per implant meaning its revocable (just delete the key in db)
	//
	//  This method is channel agnostic - not relying on a specific communication channel and should
	//  still work when using different ways of beaconing back to C2

	req.extensions_mut().insert(implant_data);
	Ok(next.run(req).await)
}



/// Small function that always returns [Error::PermissionDenied]
/// but it logs the invalid request for future inspection
fn log_err(addr: SocketAddr) -> Error {
	tracing::error!("Invalid request from {addr} on implant endpoints ! Check the logs ASAP !");
	Error::PermissionDenied
}



/// Utility function that searches for an implant using its UUID
pub async fn query_implant_by_id(
	implant_id: &Uuid,
	db: &PgPool,
) -> Result<db::Agent, (StatusCode, Json<Value>)> {
	let implant_data = sqlx::query_as!(
        db::Agent,
        r#"SELECT * FROM agents WHERE id = $1 LIMIT 1"#,
        implant_id
    )
		.fetch_one(db)
		.await
		.map_err(|e| match e {
			sqlx::error::Error::RowNotFound => (
				StatusCode::OK,
				Json(json!({"Result": format!("Implant {implant_id} not found.")})),
			),
			_ => Error::InternalError.as_tuple(),
		})?;
	Ok(implant_data)
}