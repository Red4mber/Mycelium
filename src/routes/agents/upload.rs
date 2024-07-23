use std::ops::Not;
use axum::{BoxError, Extension};
use axum::extract::{Path, Request};
use axum::http::StatusCode;
use tokio::{
	fs::File,
	io::{self, BufWriter},
};
use tokio_util::bytes::Bytes;
use tokio_util::io::StreamReader;
use futures::{Stream, TryStreamExt};
use crate::error::Error;
use crate::model::db::Agent;
use crate::settings::SETTINGS;


/// Handler for file uploads
/// It just reads
pub async fn upload_handler(
	Extension(agent): Extension<Agent>,
	Path(file_name): Path<String>,
	request: Request,
) -> Result<(), (StatusCode, String)> {
	tracing::info!("Agent {} is upload a file : {file_name} ...", agent.id);
	stream_to_file(&file_name, request.into_body().into_data_stream(), &agent)
		.await
		.map(|_x| {
			tracing::info!("File successfully uploaded !");
		})
}

/// Check if a given path only contains a single component (e.g. the file name and nothing else) \
/// Doing so, it prevent path traversal and file inclusion vulnerabilities \
/// In addition, it checks if the path is a normal, well-formed filename and doesn't contain any wildcards \
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
async fn stream_to_file<S, E>(path: &str, stream: S, agent: &Agent) -> Result<(), (StatusCode, String)>
where
	S: Stream<Item = Result<Bytes, E>>,
	E: Into<BoxError>,
{
	if path_is_valid(path).not() { 
		tracing::error!("The file cannot be uploaded, invalid path : {path}");
		return Err(Error::InvalidUploadPath.as_tuple_string()) 
	};

	async {
		// Convert the stream into an `AsyncRead`.
		let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
		let body_reader = StreamReader::new(body_with_io_error);
		futures::pin_mut!(body_reader);

		// Create the file. `File` implements `AsyncWrite`.
		let directory = std::path::Path::new(&SETTINGS.misc.uploads_dir).join(agent.id.to_string());
		std::fs::create_dir_all(&directory)?;
		let path = directory.join(path);
		let mut file = BufWriter::new(File::create(path).await?);

		// Copy the body into the file.
		io::copy(&mut body_reader, &mut file).await?;

		Ok::<_, io::Error>(())
	}
		.await
		.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}