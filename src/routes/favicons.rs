// Uses
use rocket::{fs::NamedFile, get, routes, Route, State};

use super::Routable;
use crate::config::AppConfig;

/// The route for this section.
pub(super) struct FaviconRoutes;

impl Routable for FaviconRoutes {
	const PATH: &'static str = "/";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| routes![icon_svg, icon_png, icon_ico];
}

#[get("/favicon.svg")]
pub async fn icon_svg(config: &State<AppConfig>) -> Option<NamedFile> {
	NamedFile::open(format!("{}/logo.svg", config.serve_path.as_str()))
		.await
		.ok()
}

#[get("/favicon.png")]
pub async fn icon_png(config: &State<AppConfig>) -> Option<NamedFile> {
	NamedFile::open(format!("{}/logo-tiny.png", config.serve_path.as_str()))
		.await
		.ok()
}

#[get("/favicon.ico")]
pub async fn icon_ico(config: &State<AppConfig>) -> Option<NamedFile> {
	NamedFile::open(format!("{}/favicon.ico", config.serve_path.as_str()))
		.await
		.ok()
}
