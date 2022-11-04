// Uses
use std::{ffi::OsStr, path::Path};

use rocket::{
	http::{hyper::header::CONTENT_DISPOSITION, ContentType, Header},
	request::Request,
	response::{Responder, Result as ResponseResult},
};

/// A [`Responder`] that sends data with a Content-Type and file name based on
/// an associated file name.
#[derive(Debug)]
pub struct FileFromMemory {
	file_name: String,
	contents:  Vec<u8>,
}

impl FileFromMemory {
	pub fn new(file_name: &str, contents: Vec<u8>) -> Self {
		Self {
			file_name: file_name.to_owned(),
			contents,
		}
	}

	pub fn extension(&self) -> Option<&str> {
		// Kind of hacky, but this way it can use Rust's mature implementation of
		// [`Path::extension`] instead of some custom one.
		Path::new(self.file_name.as_str())
			.extension()
			.and_then(OsStr::to_str)
	}
}

/// Streams the contents to the client, providing a Content-Type and file name.
impl<'r> Responder<'r, 'static> for FileFromMemory {
	fn respond_to(self, req: &'r Request<'_>) -> ResponseResult<'static> {
		// Figure out the content type from the file extension
		let content_type = self.extension().and_then(ContentType::from_extension);

		// Build the main response
		let mut response = self.contents.respond_to(req)?;

		// Set the Content-Type header
		if let Some(ct) = content_type {
			response.set_header(ct);
		}

		// Set the Content-Disposition header
		// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
		let content_disposition = format!("attachment; filename=\"{}\"", self.file_name);
		response.set_header(Header::new(
			CONTENT_DISPOSITION.as_str(),
			content_disposition,
		));
		
		Ok(response)
	}
}
