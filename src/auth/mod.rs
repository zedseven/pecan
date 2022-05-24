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

// The reason these two are different types and the duration needs to be defined
// twice is because there's *still* not a standard time library and everyone
// uses something different.

/// The duration of validity of the token. (before the user has to re-login)
pub const TOKEN_VALID_DURATION: &'static dyn Fn() -> ChronoDuration = &|| ChronoDuration::weeks(1);
/// The duration set on the cookie for the access token.
///
/// This is slightly shorter so that in theory users should never have a cookie
/// with a token that's not also valid on the server, under ideal circumstances.
pub const TOKEN_COOKIE_VALID_DURATION: &'static dyn Fn() -> TimeDuration =
	&|| TimeDuration::weeks(1) - TimeDuration::seconds(15);
