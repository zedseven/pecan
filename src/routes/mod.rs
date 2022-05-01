// Uses
use rocket::{fairing::AdHoc, ignite, Rocket, Route};
use rocket_contrib::serve::StaticFiles;

use crate::{
	db::{init as init_db, DbConn},
	routes::{devices::DevicesApi, svelte_pages::SveltePages},
};

// Modules
mod devices;
mod svelte_pages;

// Constants
const API_ROOT: &str = "/api";
const SVELTE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/web/build");

/// Sets up the Rocket server.
pub fn rocket() -> Rocket {
	ignite()
		.attach(AdHoc::on_attach("Database Setup", init_db))
		.attach(DbConn::fairing())
		.mount(
			format!("{}{}", API_ROOT, DevicesApi::PATH).as_str(),
			DevicesApi::ROUTES(),
		)
		.mount(SveltePages::PATH, SveltePages::ROUTES())
		.mount("/", StaticFiles::from(SVELTE_PATH))
}

/// The interface that allows a set of routes to be mounted on a path.
pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
