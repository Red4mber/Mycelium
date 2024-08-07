use tokio::io;
use std::sync::Arc;
use axum::extract::{Path, Request, State};
use axum::{BoxError, Extension};
use axum::body::Bytes;
use futures::{Stream, TryStreamExt};
use serde::Serialize;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;
use tracing::info;
use crate::{AppState, CFG};
use crate::authentication::agent::AgentData;
use crate::error::Error;
use crate::Error::InternalError;
use crate::model::{FileRecord, HostTarget};


#[derive(Serialize, Debug)]
struct FileInfo {
	from_host: Thing,
	filename: String,
	filepath: String,
}

/// Handler for file uploads
pub async fn upload_handler(
	Extension(auth): Extension<AgentData>,
	State(state): State<Arc<AppState>>,
	Path(file_name): Path<String>,
	request: Request,
) -> Result<(), Error> {
	info!("Agent {} is uploading a file : {file_name} ...", auth.record.id);

	let sql = "SELECT ->target->host FROM $agent;";
	let mut res = state.db
		.query(sql)
		.bind(("agent", auth.record.id.clone()))
		.await?;
	let host_target: Option<HostTarget> = res.take("->target")?;
	let host_id = host_target
		.ok_or(InternalError)?
		.host.first()
		.ok_or(Error::PermissionDenied)?
		.clone();

	let path = std::path::Path::new(&CFG.misc.uploads_dir)
		.join(host_id.id.to_string().trim_matches(&['⟨', '⟩']))
		.join(&file_name);

	let file_info = FileInfo {
		from_host: host_id,
		filename: file_name,
		filepath: path.to_str().unwrap().to_string(),
	};
	stream_to_file(request.into_body().into_data_stream(), &file_info).await?;

	new_file_record(&file_info, &state.db).await?;
	info!({file=?file_info}, "File successfully uploaded !");

	Ok(())
}

/// Create a new record in the database
async fn new_file_record(file_info: &FileInfo, db: &Surreal<Any>) -> Result<(), Error> {
	db.signin(Root {
		username: &CFG.db.user, password: &CFG.db.pass
	}).await?;
	db.use_ns(&CFG.db.ns).use_db(&CFG.db.db).await?;

	let rec: Vec<FileRecord> = db.insert("file")
		.content(file_info)
		.await
		.map_err(Error::DatabaseError)?;

	db.query("RELATE $host_id->upload->$file_id;")
	     .bind(("host_id", &file_info.from_host))
	     .bind(("file_id", &rec.first().unwrap().id))
	     .await?;

	Ok(())
}



/// Checks if a given path:
/// - only contains a single component
/// - the component is a well-formed filename
/// - do not contain wildcards
fn is_filename_valid(path: &str) -> bool {
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



/// Saves a `Stream` to a file
async fn stream_to_file<S, E>(stream: S, info: &FileInfo) -> Result<(), Error>
where
	S: Stream<Item = Result<Bytes, E>>,
	E: Into<BoxError>,
{
	if !is_filename_valid(info.filename.as_str()) {
		tracing::error!("The file cannot be uploaded, invalid file name : {}", &info.filename);
		return Err(Error::InvalidUploadPath)
	};

	// Convert the stream into an `AsyncRead`.
	let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
	let body_reader = StreamReader::new(body_with_io_error);
	futures::pin_mut!(body_reader);

	// Create the file. `File` implements `AsyncWrite`.
	let path = std::path::Path::new(&info.filepath);
	std::fs::create_dir_all(path.parent().unwrap()).unwrap();
	let mut file = BufWriter::new(File::create(path).await.map_err(|_| InternalError)?);

	// Copy the body into the file.
	io::copy(&mut body_reader, &mut file).await.map_err(|_| InternalError)?;

	Ok(())
}