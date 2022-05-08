// Uses
use rocket::{get, response::NamedFile, routes, Route};

use super::{Routable, SVELTE_PATH};

/// The route for this section.
pub(super) struct FaviconRoutes;
impl Routable for FaviconRoutes {
	const PATH: &'static str = "/";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| routes![icon_svg, icon_png, icon_ico];
}

#[get("/favicon.svg")]
pub fn icon_svg() -> Option<NamedFile> {
	NamedFile::open(format!("{}/logo.svg", SVELTE_PATH)).ok()
}

#[get("/favicon.png")]
pub fn icon_png() -> Option<NamedFile> {
	NamedFile::open(format!("{}/logo-tiny.png", SVELTE_PATH)).ok()
}

#[get("/favicon.ico")]
pub fn icon_ico() -> Option<NamedFile> {
	NamedFile::open(format!("{}/favicon.ico", SVELTE_PATH)).ok()
}
