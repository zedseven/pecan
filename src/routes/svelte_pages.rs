// Uses
use rocket::{fs::NamedFile, get, response::Redirect, routes, Route, State};

use super::Routable;
use crate::{auth::AuthedUserForwarding, config::AppConfig};

/// The route for this section.
pub(super) struct SveltePages;

impl Routable for SveltePages {
	const PATH: &'static str = "/";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| {
		routes![
			login_page,
			login_page_redir,
			index_page,
			index_page_redir,
			edit_page,
			edit_page_redir,
			edit_page_with_param,
			edit_page_with_param_redir
		]
	};
}

/// The login page.
#[get("/login", rank = 9)]
pub async fn login_page(config: &State<AppConfig>) -> Option<NamedFile> {
	NamedFile::open(format!("{}/login.html", config.serve_path.as_str()))
		.await
		.ok()
}

/// The index page.
///
/// Technically this isn't necessary since the static file server will
/// automatically append `index.html` to a directory, but it makes sense to
/// define it here since all other pages *will* require it.
#[get("/")]
pub async fn index_page(
	config: &State<AppConfig>,
	_user: &AuthedUserForwarding,
) -> Option<NamedFile> {
	NamedFile::open(format!("{}/index.html", config.serve_path.as_str()))
		.await
		.ok()
}

/// The edit page, without a parameter. (for adding new devices)
#[get("/edit")]
pub async fn edit_page(
	config: &State<AppConfig>,
	_user: &AuthedUserForwarding,
) -> Option<NamedFile> {
	NamedFile::open(format!("{}/edit.html", config.serve_path.as_str()))
		.await
		.ok()
}

/// The edit page, with a parameter. (for editing existing devices)
///
/// Svelte handles the actual parameter, so we need not worry about it here.
#[get("/edit/<_device>")]
pub async fn edit_page_with_param(
	config: &State<AppConfig>,
	user: &AuthedUserForwarding,
	_device: String,
) -> Option<NamedFile> {
	edit_page(config, user).await
}

// Auth redirect catchers - these redirect based on whether the user is logged
// in or not
fn redirect_to_login() -> Redirect {
	Redirect::to(uri!("/login"))
}

fn redirect_to_index() -> Redirect {
	Redirect::to(uri!("/"))
}

/// The login page.
#[get("/login")]
pub fn login_page_redir(_user: &AuthedUserForwarding) -> Redirect {
	redirect_to_index()
}

#[get("/", rank = 9)]
pub fn index_page_redir() -> Redirect {
	redirect_to_login()
}

#[get("/edit", rank = 9)]
pub fn edit_page_redir() -> Redirect {
	redirect_to_login()
}

#[get("/edit/<_device>", rank = 9)]
pub fn edit_page_with_param_redir(_device: String) -> Redirect {
	redirect_to_login()
}
