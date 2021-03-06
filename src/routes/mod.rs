// Uses
use rocket::{
	fairing::AdHoc,
	figment::{
		providers::{Env, Format, Serialized, Toml},
		Figment,
		Profile,
	},
	fs::FileServer,
	Build,
	Config,
	Rocket,
	Route,
};

use crate::{
	auth::LdapAuthenticator,
	config::{
		AppConfig,
		LdapSettings,
		CONFIG_ENV_PREFIX,
		CONFIG_FILE_ENV_OVERRIDE,
		CONFIG_FILE_NAME,
		CONFIG_FILE_PROFILE_ENV_NAME,
	},
	db::{init as init_db, DbConn},
	routes::{admin::AdminApi, auth::AuthApi, devices::DevicesApi, svelte_pages::SveltePages},
};

// Modules
mod admin;
mod auth;
mod devices;
mod svelte_pages;

// Constants
const API_ROOT: &str = "/api";

/// Sets up the Rocket server.
pub fn rocket() -> Rocket<Build> {
	// Load the config
	let config = Figment::from(Config::default())
		.join(Serialized::defaults(AppConfig::default()))
		.merge(Toml::file(Env::var_or(CONFIG_FILE_ENV_OVERRIDE, CONFIG_FILE_NAME)).nested())
		.merge(
			Env::prefixed(CONFIG_ENV_PREFIX)
				.ignore(&["PROFILE"])
				.global(),
		)
		.select(Profile::from_env_or(
			CONFIG_FILE_PROFILE_ENV_NAME,
			Config::DEFAULT_PROFILE,
		));

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
						eprintln!("{}", e);
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
			format!("{}{}", API_ROOT, AuthApi::PATH).as_str(),
			AuthApi::ROUTES(),
		)
		.mount(
			format!("{}{}", API_ROOT, DevicesApi::PATH).as_str(),
			DevicesApi::ROUTES(),
		)
		.mount(
			format!("{}{}", API_ROOT, AdminApi::PATH).as_str(),
			AdminApi::ROUTES(),
		)
		.mount(SveltePages::PATH, SveltePages::ROUTES())
		.mount("/", FileServer::from(svelte_path.as_str()));

	rocket
}

#[allow(clippy::unused_async)]
async fn validate_config(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
	// The `unwrap`s in this function are okay because Figment validates that the
	// values are present anyways.

	if rocket
		.figment()
		.extract_inner::<u32>("token_valid_days")
		.unwrap()
		< 1
	{
		eprintln!("token_valid_days must be a positive value");
		return Err(rocket);
	}

	Ok(rocket)
}

/// The interface that allows a set of routes to be mounted on a path.
pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
