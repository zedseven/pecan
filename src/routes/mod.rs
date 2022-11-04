// Uses
use rocket::{fairing::AdHoc, fs::FileServer, Build, Rocket, Route};

use crate::{
	auth::LdapAuthenticator,
	config::{load_complete_config, validate_config, AppConfig, LdapSettings},
	db::{init as init_db, DbConn},
	routes::{admin::AdminApi, auth::AuthApi, devices::DevicesApi, svelte_pages::SveltePages},
};

// Modules
mod admin;
mod auth;
mod devices;
mod file_from_memory;
mod svelte_pages;

// Constants
const API_ROOT: &str = "/api";

/// Sets up the Rocket server.
pub fn rocket() -> Rocket<Build> {
	// Load the config
	let config = load_complete_config();

	// Prepare for launch
	let mut rocket = Rocket::custom(config)
		.attach(AdHoc::config::<AppConfig>())
		.attach(AdHoc::try_on_ignite("Config Validation", validate_config))
		.attach(DbConn::fairing())
		.attach(AdHoc::try_on_ignite("Database Setup", init_db));

	// Fetch the Svelte path
	let svelte_path = rocket
		.figment()
		.extract_inner::<String>("serve_path")
		.expect("missing required configuration parameter `serve_path`");

	// Prepare the LDAP authenticator if LDAP is enabled
	let mut ldap_enabled = false;
	if let Ok(ldap_config) = rocket.figment().extract_inner::<LdapSettings>("ldap") {
		if ldap_config.enabled {
			ldap_enabled = true;
			rocket = rocket.attach(AdHoc::try_on_ignite("LDAP Authenticator", async move |r| {
				match LdapAuthenticator::try_from(&ldap_config) {
					Ok(auth) => Ok(r.manage(Some(auth))),
					Err(e) => {
						eprintln!("{e}");
						Err(r)
					}
				}
			}));
		}
	}
	if !ldap_enabled {
		rocket = rocket.manage::<Option<LdapAuthenticator>>(None);
	}

	// Mount the routes
	rocket = rocket
		.mount(
			format!("{API_ROOT}{}", AuthApi::PATH).as_str(),
			AuthApi::ROUTES(),
		)
		.mount(
			format!("{API_ROOT}{}", DevicesApi::PATH).as_str(),
			DevicesApi::ROUTES(),
		)
		.mount(
			format!("{API_ROOT}{}", AdminApi::PATH).as_str(),
			AdminApi::ROUTES(),
		)
		.mount(SveltePages::PATH, SveltePages::ROUTES())
		.mount("/", FileServer::from(svelte_path.as_str()));

	rocket
}

/// The interface that allows a set of routes to be mounted on a path.
pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
