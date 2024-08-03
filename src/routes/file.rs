#![allow(unused)]
use tokio::io;
use std::ops::Not;
use std::str::FromStr;
use std::sync::Arc;
use axum::extract::{Path, Request, State};
use axum::middleware::from_fn_with_state;
use axum::{BoxError, Extension, Json, Router};
use axum::body::Bytes;
use axum::http::StatusCode;
use axum::routing::{get, post};
use futures::{Stream, TryStreamExt};
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;
use uuid::Uuid;
use crate::{AppState, CFG};
use crate::authentication::{agent_middleware, auth_middleware};
use crate::authentication::agent::AgentData;
use crate::error::Error;
use crate::Error::InternalError;
use crate::model::{FileRecord, HostRecord};
use crate::model::auth::AuthData;


/// Returns all the routes of this module
pub fn get_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", get(file_query_all))
		.layer(from_fn_with_state(app_state.clone(), auth_middleware))
		.route("/upload/:file_name", post(upload_handler).layer(from_fn_with_state(app_state.clone(), agent_middleware)))
		.with_state(app_state)
}

/// Route used to list every file in the database
pub async fn file_query_all(
	State(state): State<Arc<AppState>>
) -> Result<Json<Value>, String> {
	let res: Vec<FileRecord> = state.db.select("file").await.map_err(|e|e.to_string())?;
	Ok(Json(json!(res)))
}





/// Handler for file uploads
/// It just reads
pub async fn upload_handler(
	Extension(auth): Extension<AgentData>,
	Path(file_name): Path<String>,
	request: Request,
) -> Result<(), (StatusCode, String)> {
	tracing::info!("Agent {} is uploading a file : {file_name} ...", auth.record.id);
	stream_to_file(&file_name, request.into_body().into_data_stream(), auth.record.host.id.to_string().as_str())
		.await
		.map(|_x| {
			tracing::info!("File successfully uploaded !");
		});
	Ok(())
}

/// Checks if a given path:
/// - only contains a single component
/// - the component is a well-formed filename
/// - do not contain wildcards
fn path_is_valid(path: &str) -> bool {
	let path = std::path::Path::new(path);
	let mut components = path.components().peekable();
	if let Some(first) = components.peek() {
		// Checks if component is a normal file name
		if !matches!(first, std::path::Component::Normal(_)) {
			return false;
		}
	}
	components.count().eq(&1)
}



// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S, host_id: &str) -> Result<(), Error>
where
	S: Stream<Item = Result<Bytes, E>>,
	E: Into<BoxError>,
{
	if path_is_valid(path).not() {
		tracing::error!("The file cannot be uploaded, invalid path : {path}");
		return Err(Error::InvalidUploadPath)
	};

	async {
		// Convert the stream into an `AsyncRead`.
		let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
		let body_reader = StreamReader::new(body_with_io_error);
		futures::pin_mut!(body_reader);

		// Create the file. `File` implements `AsyncWrite`.
		let directory = std::path::Path::new(&CFG.misc.uploads_dir).join(host_id);
		std::fs::create_dir_all(&directory)?;
		let path = directory.join(path);
		let mut file = BufWriter::new(File::create(path).await?);

		// Copy the body into the file.
		io::copy(&mut body_reader, &mut file).await?;

		Ok(Ok(()))
	}
	.await.map_err(|err: tokio::io::Error| InternalError)?
}