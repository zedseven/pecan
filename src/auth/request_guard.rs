// Uses
use chrono::Utc;
use diesel::{
	result::Error as DieselError,
	ExpressionMethods,
	OptionalExtension,
	QueryDsl,
	RunQueryDsl,
	SqliteConnection,
};
use rocket::{
	http::{Cookie, Status},
	request::{FromRequest, Outcome},
	Request,
};

use super::COOKIE_NAME;
use crate::db::{
	models::{User, USER},
	schema,
	DbConn,
};

/// The request guard that verifies the user has a valid login token.
pub struct AuthedUser(User);

/// Identical to [`AuthedUser`], but it forwards on failure. This allows for
/// Rocket to serve redirects on user pages that require authentication.
pub struct AuthedUserForwarding(User);

/// The error type for [`AuthedUser`] failures.
#[derive(Debug, Copy, Clone)]
pub enum AuthedUserError {
	MissingCookie,
	InvalidToken,
	DatabaseError,
}

// Implementation
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AuthedUser {
	type Error = AuthedUserError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let result = request
			.local_cache_async(async { validate_cookie(request, AuthedUser).await })
			.await;

		match result {
			Ok(user) => Outcome::Success(user),
			Err(e @ (AuthedUserError::MissingCookie | AuthedUserError::InvalidToken)) => {
				Outcome::Failure((Status::BadRequest, *e))
			}
			Err(e @ AuthedUserError::DatabaseError) => {
				Outcome::Failure((Status::InternalServerError, *e))
			}
		}
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AuthedUserForwarding {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let result = request
			.local_cache_async(async { validate_cookie(request, AuthedUserForwarding).await })
			.await;

		match result {
			Ok(user) => Outcome::Success(user),
			Err(_) => Outcome::Forward(()),
		}
	}
}

/// Does the actual cookie validation.
async fn validate_cookie<T, F>(
	request: &Request<'_>,
	build_return_value: F,
) -> Result<T, AuthedUserError>
where
	F: FnOnce(User) -> T,
{
	match request.cookies().get_private(COOKIE_NAME) {
		None => {
			remove_cookie(request);
			Err(AuthedUserError::MissingCookie)
		}
		Some(cookie) => {
			let conn = request
				.guard::<DbConn>()
				.await
				.succeeded()
				.ok_or(AuthedUserError::DatabaseError)?;

			let cloned_cookie = cookie.clone();
			let validation_result = conn
				.run(move |c| token_is_valid(c, cloned_cookie.value()))
				.await
				.map_err(|_| AuthedUserError::DatabaseError)?;

			if let Some(user) = validation_result {
				Ok(build_return_value(user))
			} else {
				remove_cookie(request);
				Err(AuthedUserError::InvalidToken)
			}
		}
	}
}

/// Verifies that a token is valid.
fn token_is_valid(conn: &mut SqliteConnection, token: &str) -> Result<Option<User>, DieselError> {
	use schema::{tokens::dsl::*, user_info::dsl::*};

	let user_result = tokens
		.inner_join(user_info)
		.filter(value.eq(token))
		.filter(expires.gt(Utc::now().naive_utc()))
		.filter(valid.eq(true))
		.select(USER)
		.get_result::<User>(conn)
		.optional()?;

	Ok(user_result)
}

/// Removes the token cookie if present.
fn remove_cookie(request: &Request<'_>) {
	request.cookies().remove_private(Cookie::named(COOKIE_NAME));
}
