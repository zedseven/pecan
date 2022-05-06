//! Effectively a custom [`anyhow`] implementation, but with Rocket's
//! [`Responder`] support.
//!
//! It's not quite as generic and full-featured as `anyhow`, but it doesn't need
//! to be, and it has the additional Rocket support instead.
//!
//! [`anyhow`]: https://docs.rs/anyhow

// Uses
use rocket::{
	http::Status,
	response::{
		self,
		status::{BadRequest, NotFound},
		Responder,
	},
	Request,
};
use thiserror::Error;

/// The internal error type, with Rocket's [`Responder`] implemented to make it
/// ergonomic to use in routes.
#[derive(Error, Debug)]
pub enum InternalError {
	#[error("assertion failed: {0}")]
	Assertion(&'static str),
	#[error("internal database error: {0}")]
	Diesel(#[from] diesel::result::Error),
}

impl InternalError {
	#[must_use]
	pub fn with_context<S>(self, context: S) -> Error
	where
		S: Into<String>,
	{
		Error::WithContext {
			source: self,
			context: context.into(),
		}
	}
}

impl From<&'static str> for InternalError {
	fn from(s: &'static str) -> Self {
		Self::Assertion(s)
	}
}

/// The internal error type, with Rocket's [`Responder`] implemented to make it
/// ergonomic to use in routes.
#[derive(Error, Debug)]
pub enum UserError {
	#[error("{0}")]
	BadRequest(&'static str),
	#[error("{0}")]
	NotFound(&'static str),
}

/// The [`InternalError`] type, with context.
#[derive(Error, Debug)]
pub enum Error {
	/// User errors with *a message to be displayed to the user*.
	#[error("user error: {0}")]
	User(#[from] UserError),
	/// Internal errors without context.
	#[error(transparent)]
	NoContext(#[from] InternalError),
	/// Internal errors with context.
	#[error("{context}: {source}")]
	WithContext {
		source: InternalError,
		context: String,
	},
}

impl Error {
	#[must_use]
	pub fn with_context<S>(self, context: S) -> Error
	where
		S: Into<String>,
	{
		#[allow(
			clippy::wildcard_enum_match_arm,
			clippy::match_wildcard_for_single_variants
		)]
		match self {
			Self::NoContext(source) => Self::WithContext {
				source,
				context: context.into(),
			},
			Self::WithContext {
				source,
				context: internal_context,
			} => Self::WithContext {
				source,
				context: format!("{}: {}", context.into(), internal_context),
			},
			// Throw out the context if the error variant is unsupported
			_ => self,
		}
	}
}

impl<T> From<T> for Error
where
	T: Into<InternalError>,
{
	// Currently requires the unstable feature for specialization
	default fn from(s: T) -> Self {
		Self::NoContext(s.into())
	}
}

/// A trait providing the ability to easily add context to an error.
pub trait Context<T, E> {
	fn with_context<S>(self, context: S) -> Result<T, Error>
	where
		S: Into<String>;
}

impl<T, E> Context<T, E> for Result<T, E>
where
	E: Into<InternalError>,
{
	fn with_context<S>(self, context: S) -> Result<T, Error>
	where
		S: Into<String>,
	{
		self.map_err(|err| err.into().with_context(context))
	}
}

impl<T> Context<T, Error> for Result<T, Error> {
	fn with_context<S>(self, context: S) -> Result<T, Error>
	where
		S: Into<String>,
	{
		self.map_err(|err| err.with_context(context))
	}
}

// Responder Implementations
impl<'r> Responder<'r> for InternalError {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		eprintln!("{}", self);
		Status::InternalServerError.respond_to(request)
	}
}

impl<'r> Responder<'r> for UserError {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		match self {
			UserError::BadRequest(message) => BadRequest(Some(message)).respond_to(request),
			UserError::NotFound(message) => NotFound(Some(message)).respond_to(request),
		}
	}
}

// Forward to the internal error type
impl<'r> Responder<'r> for Error {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		match self {
			Error::User(err) => err.respond_to(request),
			Error::NoContext(err) | Error::WithContext { source: err, .. } => {
				err.respond_to(request)
			}
		}
	}
}
