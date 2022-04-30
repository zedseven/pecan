// Uses
use rocket::{fairing::AdHoc, ignite, Rocket, Route};
use rocket_contrib::serve::StaticFiles;

use crate::{
	db::{init as init_db, DbConn},
	routes::api::Api,
};

// Modules
mod api;

/// Sets up the Rocket server.
pub fn rocket() -> Rocket {
	ignite()
		.attach(DbConn::fairing())
		.attach(AdHoc::on_attach("Database Setup", init_db))
		.mount(Api::PATH, Api::ROUTES())
		.mount(
			"/",
			StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/web/build")),
		)
}

/// The interface that allows a set of routes to be mounted on a path.
pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
