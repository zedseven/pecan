// Uses
use rocket::{fs::NamedFile, get, routes, Route};

use super::{Routable, SVELTE_PATH};

/// The route for this section.
pub(super) struct SveltePages;
impl Routable for SveltePages {
	const PATH: &'static str = "/";
	const ROUTES: &'static dyn Fn() -> Vec<Route> =
		&|| routes![index_page, edit_page, edit_page_with_param, admin_page];
}

/// The index page.
///
/// Technically this isn't necessary since the static file server will
/// automatically append `index.html` to a directory, but it makes sense to
/// define it here since all other pages *will* require it.
#[get("/")]
pub async fn index_page() -> Option<NamedFile> {
	NamedFile::open(format!("{}/index.html", SVELTE_PATH))
		.await
		.ok()
}

/// The edit page, without a parameter. (for adding new devices)
#[get("/edit")]
pub async fn edit_page() -> Option<NamedFile> {
	NamedFile::open(format!("{}/edit.html", SVELTE_PATH))
		.await
		.ok()
}

/// The edit page, with a parameter. (for editing existing devices)
///
/// Svelte handles the actual parameter, so we need not worry about it here.
#[get("/edit/<_device>")]
pub async fn edit_page_with_param(_device: String) -> Option<NamedFile> {
	edit_page().await
}

/// The admin page.
#[get("/admin")]
pub async fn admin_page() -> Option<NamedFile> {
	NamedFile::open(format!("{}/admin.html", SVELTE_PATH))
		.await
		.ok()
}
