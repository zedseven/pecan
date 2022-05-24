// Uses
use chrono::Duration as ChronoDuration;
use rocket::time::Duration as TimeDuration;

// Exports
pub use self::{authenticator::*, request_guard::*, tokens::*};

// Modules
mod authenticator;
mod request_guard;
mod tokens;

// Constants
/// The name of the cookie used to store the access token.
pub const COOKIE_NAME: &str = "access_token";

// The reason these two are different types and the duration needs two different
// functions is because there's *still* not a standard time library and everyone
// uses something different.

/// The duration of validity of the token. (before the user has to re-login)
pub fn get_token_valid_duration(days: u32) -> ChronoDuration {
	ChronoDuration::days(i64::from(days))
}

/// The duration set on the cookie for the access token.
///
/// This is slightly shorter so that in theory users should never have a cookie
/// with a token that's not also valid on the server, under ideal circumstances.
pub fn get_token_cookie_valid_duration(days: u32) -> TimeDuration {
	TimeDuration::days(i64::from(days)) - TimeDuration::seconds(15)
}
