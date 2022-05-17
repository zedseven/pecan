// Uses
use rocket::{build, fairing::AdHoc, fs::FileServer, Build, Rocket, Route};

use crate::{
	db::{init as init_db, DbConn},
	routes::{
		admin::AdminApi,
		devices::DevicesApi,
		favicons::FaviconRoutes,
		svelte_pages::SveltePages,
	},
};

// Modules
mod admin;
mod devices;
mod favicons;
mod svelte_pages;

// Constants
const API_ROOT: &str = "/api";
const SVELTE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/web/build");

/// Sets up the Rocket server.
pub fn rocket() -> Rocket<Build> {
	build()
		.attach(DbConn::fairing())
		.attach(AdHoc::on_ignite("Database Setup", init_db))
		.mount(
			format!("{}{}", API_ROOT, DevicesApi::PATH).as_str(),
			DevicesApi::ROUTES(),
		)
		.mount(
			format!("{}{}", API_ROOT, AdminApi::PATH).as_str(),
			AdminApi::ROUTES(),
		)
		.mount(FaviconRoutes::PATH, FaviconRoutes::ROUTES())
		.mount(SveltePages::PATH, SveltePages::ROUTES())
		.mount("/", FileServer::from(SVELTE_PATH))
}

/// The interface that allows a set of routes to be mounted on a path.
pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
