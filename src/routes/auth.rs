// Uses
use rocket::{
	http::{Cookie, CookieJar},
	post,
	routes,
	serde::json::Json,
	time::OffsetDateTime,
	Route,
	State,
};

use super::Routable;
use crate::{
	auth::{
		generate_token_for_user,
		get_token_cookie_valid_duration,
		AuthedUserForwarding,
		LdapAuthenticator,
		COOKIE_NAME,
	},
	config::AppConfig,
	db::DbConn,
	error::{Context, Error, UserError},
};

/// The route for this section.
pub(super) struct AuthApi;

impl Routable for AuthApi {
	const PATH: &'static str = "/";
	const ROUTES: &'static dyn Fn() -> Vec<Route> =
		&|| routes![authenticate, logged_in_true, logged_in_false];
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthData {
	username: String,
	password: String,
}

#[post("/authenticate", data = "<auth_data>")]
pub async fn authenticate(
	config: &State<AppConfig>,
	authenticator: &State<LdapAuthenticator>,
	cookie_jar: &CookieJar<'_>,
	conn: DbConn,
	auth_data: Json<AuthData>,
) -> Result<Json<()>, Error> {
	// Authenticate the credentials with the server
	let auth_result = authenticator
		.authenticate_user(
			auth_data.username.as_str().trim(),
			auth_data.password.as_str(),
		)
		.await
		.with_context("something went wrong when attempting to authenticate a user")?;

	if auth_result.is_none() {
		return Err(UserError::BadRequest("Invalid credentials.").into());
	}
	// let user_dn = auth_result.unwrap();

	// If auth was successful, generate the new token and set a cookie for the user
	let token_valid_days = config.token_valid_days;
	let username_clone = auth_data.username.clone();
	let new_token = conn
		.run(move |c| generate_token_for_user(c, username_clone.as_str().trim(), token_valid_days))
		.await
		.with_context("failed to generate the new token")?;
	let mut new_cookie = Cookie::new(COOKIE_NAME, new_token);
	new_cookie
		.set_expires(OffsetDateTime::now_utc() + get_token_cookie_valid_duration(token_valid_days));
	cookie_jar.add_private(new_cookie);

	Ok(Json(()))
}

#[get("/loggedIn")]
pub fn logged_in_true(_user: &AuthedUserForwarding) -> Json<bool> {
	Json(true)
}

#[get("/loggedIn", rank = 9)]
pub fn logged_in_false() -> Json<bool> {
	Json(false)
}
