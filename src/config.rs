// Constants
pub const CONFIG_FILE_NAME: &str = "pecan.toml";
pub const CONFIG_FILE_ENV_OVERRIDE: &str = "PECAN_CONFIG";
pub const CONFIG_ENV_PREFIX: &str = "PECAN_";

// Config Struct

/// The app configuration.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppConfig {
	/// Settings for LDAP-based authentication.
	pub ldap: Option<LdapSettings>,
}

/// Settings for LDAP-based authentication.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LdapSettings {
	/// Whether LDAP is enabled.
	///
	/// If the LDAP settings are missing, it's treated as though this is not
	/// enabled.
	pub enabled: bool,
	/// The server to connect to, including the LDAP scheme, IP address/domain,
	/// and port.
	///
	/// For example:
	/// - `ldap://127.0.0.1:389`
	/// - `ldaps://127.0.0.1:636`
	///
	/// *Note that if using LDAPS (LDAP over SSL), TLS must be enabled.*
	pub server_url: String,
	/// The server type - either plain LDAP, or Active Directory (still served
	/// over LDAP).
	///
	/// This is important because AD uses a bit of a different schema.
	pub r#type: LdapServerType,
	/// LDAP TLS settings.
	#[serde(default)]
	pub tls: LdapTlsSettings,
	/// The settings for the read-only reader user.
	///
	/// This user is used to search for users and perform authentication.
	pub reader: LdapReaderSettings,
	/// The base to start the search for users from.
	///
	/// This should be as far down the tree as possible.
	pub search_base: String,
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
	pub enabled: bool,
	/// Whether to verify TLS certificates from the server.
	///
	/// This should be true unless the LDAPS server has certificate problems.
	pub verify_certificates: bool,
}

impl Default for LdapTlsSettings {
	fn default() -> Self {
		Self {
			enabled: false,
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
	pub password: String,
}
