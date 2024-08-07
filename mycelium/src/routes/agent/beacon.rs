use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use tracing::info;
use crate::{AppState, CFG};
use crate::authentication::agent::AgentData;
use crate::model::{AgentRecord, BeaconData, HostRecord, HostTarget, NoIdAgentRecord};
use crate::routes::agent::create_host_record;


// TODO CLEANUP THIS FUNCTION
/// Receive beacon data
pub(crate) async fn beacon_handler(
	Extension(auth): Extension<AgentData>,
	State(state): State<Arc<AppState>>,
	Json(beacon_data): Json<BeaconData>,
) -> Result<(), crate::error::Error> {
	state.db.signin(Root { username: &CFG.db.user, password: &CFG.db.pass }).await?;
	state.db.use_ns(&CFG.db.ns).use_db(&CFG.db.db).await?;

	info!({id=auth.record.id.to_string()/*,data=?beacon_data*/}, "Receiving beacon.");

	let sql = "SELECT ->target->host FROM $agent;";
	let mut res = state.db
		.query(sql)
		.bind(("agent", auth.record.id.clone()))
		.await?;
	let host_target: Option<HostTarget> = res.take("->target")?;
	// debug!("{host_id:#?}");

	// Unwrap OK > Always Some, even if there's no results
	if host_target.clone().unwrap().host.is_empty() {
		// Create Host record
		let new_id = uuid::Uuid::new_v4().to_string();
		let _res: Option<HostRecord> = state.db
			.insert(("host", new_id.clone()))
			.content(create_host_record(&beacon_data))
			.await?;

		// Update Agent record
		let agent_rec = auth.record.clone();
		let host_id = Thing::from(("host".to_string(), new_id.clone()));
		let _res: Option<AgentRecord> = state.db
			.update(&agent_rec.id)
			.content(NoIdAgentRecord {
				time: auth.record.time.clone(),
				key: auth.record.key.clone(),
			})
			.await?;
		// debug!("{agent_rec:#?}");

		// Relate Agent to Host
		state.db.query("RELATE $agent_id->target->$host_id;")
			.bind(("agent_id", auth.record.id.clone()))
			.bind(("host_id", host_id.clone()))
			.await?;
	} else {
		let host_id = host_target.unwrap().host.first().unwrap().clone();
		let _res: Option<HostRecord> = state.db
			.update(host_id)
			.content(create_host_record(&beacon_data))
			.await?;
	}

	Ok(())
}