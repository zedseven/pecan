// Uses
use rocket::{
	data::{ByteUnit, Limits, ToByteUnit},
	figment::{
		providers::{Env, Format, Serialized, Toml},
		Figment,
		Profile,
	},
	Build,
	Config as RocketConfig,
	Rocket,
};

// Constants
const CONFIG_FILE_NAME: &str = "pecan.toml";
const CONFIG_FILE_ENV_OVERRIDE: &str = "PECAN_CONFIG";
const CONFIG_FILE_PROFILE_ENV_NAME: &str = "PECAN_PROFILE";
const CONFIG_ENV_PREFIX: &str = "PECAN_";
const RELEASE_DIST_PATH: &str = "dist";
const DEFAULT_JSON_LIMIT: &'static dyn Fn() -> ByteUnit = &|| 5.mebibytes();

// Config Struct

/// The app configuration.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppConfig {
	/// The path to the directory to serve the front-end Svelte files from.
	pub serve_path:          String,
	/// How many days a login token is valid for, before a user has to log in
	/// again.
	pub token_valid_days:    u32,
	/// The maximum attachment size allowed on upload.
	pub max_attachment_size: ByteUnit,
	/// Settings for LDAP-based authentication.
	pub ldap:                Option<LdapSettings>,
}

impl Default for AppConfig {
	fn default() -> Self {
		Self {
			serve_path:          if cfg!(debug_assertions) {
				concat!(env!("CARGO_MANIFEST_DIR"), "/web/build")
			} else {
				RELEASE_DIST_PATH
			}
			.to_owned(),
			token_valid_days:    7,
			max_attachment_size: 3.mebibytes(),
			ldap:                None,
		}
	}
}

/// Settings for LDAP-based authentication.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LdapSettings {
	/// Whether LDAP is enabled.
	///
	/// If the LDAP settings are missing, it's treated as though this is not
	/// enabled.
	pub enabled:                     bool,
	/// The server to connect to, including the LDAP scheme, IP address/domain,
	/// and port.
	///
	/// For example:
	/// - `ldap://127.0.0.1:389`
	/// - `ldaps://127.0.0.1:636`
	///
	/// *Note that if using LDAPS (LDAP over SSL), TLS must be enabled.*
	pub server_url:                  String,
	/// The server type - either plain LDAP, or Active Directory (still served
	/// over LDAP).
	///
	/// This is important because AD uses a bit of a different schema.
	pub r#type:                      LdapServerType,
	/// LDAP TLS settings.
	#[serde(default)]
	pub tls:                         LdapTlsSettings,
	/// The settings for the read-only reader user.
	///
	/// This user is used to search for users and perform authentication.
	pub reader:                      LdapReaderSettings,
	/// The bases to start the search for users from.
	///
	/// Each one should be as far down the tree as possible. (not the whole
	/// system, just the specific groups who need access)
	pub search_bases:                Vec<String>,
	/// The LDAP attribute that stores the user display name. (first & last
	/// name)
	pub user_display_name_attribute: String,
}

/// The type of LDAP server that's being connected-to.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LdapServerType {
	#[serde(rename = "LDAP")]
	Ldap,
	#[serde(rename = "AD")]
	ActiveDirectory,
}

impl Default for LdapServerType {
	fn default() -> Self {
		Self::Ldap
	}
}

/// LDAP TLS settings.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LdapTlsSettings {
	/// Whether TLS is enabled or not.
	///
	/// This must be enabled if LDAPS (LDAP over SSL) is in use.
	pub enabled:             bool,
	/// Whether to verify TLS certificates from the server.
	///
	/// This should be true unless the LDAPS server has certificate problems.
	pub verify_certificates: bool,
}

impl Default for LdapTlsSettings {
	fn default() -> Self {
		Self {
			enabled:             false,
			verify_certificates: true,
		}
	}
}

/// The settings for the read-only reader user.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LdapReaderSettings {
	/// The complete Distinguished Name (DN) of the user.
	pub distinguished_name: String,
	/// The password for binding to the user.
	pub password:           String,
}

/// Builds and loads the complete configuration for the program.
pub fn load_complete_config() -> Figment {
	Figment::from(RocketConfig::default())
		.join(Serialized::defaults(AppConfig::default()))
		.merge((
			"limits",
			Limits::default().limit("json", DEFAULT_JSON_LIMIT()),
		))
		.merge(Toml::file(Env::var_or(CONFIG_FILE_ENV_OVERRIDE, CONFIG_FILE_NAME)).nested())
		.merge(
			Env::prefixed(CONFIG_ENV_PREFIX)
				.ignore(&["PROFILE"])
				.global(),
		)
		.select(Profile::from_env_or(
			CONFIG_FILE_PROFILE_ENV_NAME,
			RocketConfig::DEFAULT_PROFILE,
		))
}

/// Validates config settings. This is in a separate step from
/// [`load_complete_config`] so that it can be included in Rocket's fairings.
/// This makes it so that the error message is easily visible.
#[allow(clippy::unused_async)]
pub async fn validate_config(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
	if rocket
		.figment()
		.extract_inner::<u32>("token_valid_days")
		.expect("figment ensures the value is present")
		< 1
	{
		eprintln!("token_valid_days must be a positive value");
		return Err(rocket);
	}

	Ok(rocket)
}
