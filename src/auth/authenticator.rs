// Uses
use std::cmp::Ordering;

use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};

use crate::{
	config::{LdapServerType, LdapSettings},
	db::{enums::UserSource, models::UserNew},
	error::InternalError,
};

/// The object that handles authentication with an LDAP server.
#[derive(Debug)]
pub struct LdapAuthenticator {
	/// The server URL to connect to, including the scheme definition and the
	/// port.
	///
	/// For example: `ldaps://localhost:2636` to connect to an LDAP over SSL
	/// server on port 2636.
	pub server_url: String,
	/// Whether to use TLS to connect to the server. To do this, the server must
	/// support TLS. (such as an LDAP over SSL server, or LDAPS)
	pub use_tls: bool,
	/// Whether to perform certificate verification. Does not apply if `use_tls`
	/// is `false`.
	pub verify_tls_certificate: bool,
	/// The Distinguished Name of the reader user.
	pub reader_dn: String,
	/// The password to bind to the reader user.
	pub reader_password: String,
	/// The bases to search in when searching a provided username.
	pub search_bases: Vec<String>,
	/// The LDAP attribute used to identify users. (what is searched-for when a
	/// user provides their username)
	///
	/// This varies depending on whether it's a plain LDAP server, or an Active
	/// Directory server. LDAP uses `uid`, but AD uses `sAMAccountName` (user
	/// identifier) or `userPrincipalName` (user identifier @ domain)
	pub user_identifier_attribute: String,
	/// The LDAP attribute that stores the user display name.
	pub display_name_attribute: String,
}

/// The return value of an LDAP authentication.
pub struct AuthenticationReturn {
	/// The DN of the user.
	distinguished_name: String,
	/// The `uid` on LDAP, and `sAMAccountName` on AD.
	unique_identifier: String,
	/// The display name for the user. Typically first & last name.
	display_name: String,
}

impl TryFrom<&LdapSettings> for LdapAuthenticator {
	type Error = InternalError;

	fn try_from(config: &LdapSettings) -> Result<Self, Self::Error> {
		// Config validation
		if config.server_url.is_empty()
			|| config.reader.distinguished_name.is_empty()
			|| config.search_bases.is_empty()
			|| config.user_display_name_attribute.is_empty()
		{
			return Err("one or more values is empty".into());
		}
		if !config.tls.enabled && config.server_url.starts_with("ldaps:") {
			return Err("TLS is disabled but the specified URL uses LDAP over SSL".into());
		}

		// Load the config
		Ok(Self {
			server_url: config.server_url.clone(),
			use_tls: config.tls.enabled,
			verify_tls_certificate: config.tls.verify_certificates,
			reader_dn: config.reader.distinguished_name.clone(),
			reader_password: config.reader.password.clone(),
			search_bases: config.search_bases.clone(),
			user_identifier_attribute: match config.r#type {
				LdapServerType::Ldap => "uid",
				LdapServerType::ActiveDirectory => "sAMAccountName",
			}
			.to_owned(),
			display_name_attribute: config.user_display_name_attribute.clone(),
		})
	}
}

impl LdapAuthenticator {
	/// Authenticate a user against the server.
	///
	/// Returns an `Err` if there was an operational problem.
	/// The `Ok(Option)` will indicate if authentication was successful or not.
	///
	/// If the `Option` is `Some`, the authentication was valid and the
	/// contained value is the user's distinguished name on the server.
	pub async fn authenticate_user(
		&self,
		mut username: &str,
		password: &str,
	) -> Result<Option<AuthenticationReturn>, &'static str> {
		// Validate the provided username - it can't contain certain special characters
		// TODO: This could be improved to escape the characters instead of failing
		// https://cheatsheetseries.owasp.org/cheatsheets/LDAP_Injection_Prevention_Cheat_Sheet.html
		// https://datatracker.ietf.org/doc/html/rfc4515#section-3
		// https://wiki.sei.cmu.edu/confluence/spaces/flyingpdf/pdfpageexport.action?pageId=88487534
		// The `@` is also excluded because email address logins aren't supported.
		if username.contains([
			'\'', '"', '*', '+', '(', ')', '/', '\\', '<', '>', ',', ';', '\u{0000}', '@',
		]) {
			return Ok(None);
		}

		username = username.trim();

		if username.starts_with('#') {
			return Ok(None);
		}

		// Start the connection
		let (conn, mut ldap) = LdapConnAsync::with_settings(
			LdapConnSettings::new()
				.set_starttls(self.use_tls)
				.set_no_tls_verify(!self.verify_tls_certificate),
			self.server_url.as_str(),
		)
		.await
		.map_err(|_| "unable to establish a connection to the server")?;
		ldap3::drive!(conn);

		// Connect to the reader user
		ldap.simple_bind(self.reader_dn.as_str(), self.reader_password.as_str())
			.await
			.map_err(|_| "unable to attempt a bind operation")?
			.success()
			.map_err(|_| "unable to bind to the reader user")?;

		// Search for the given username in the list of search bases
		// The first single user found in a search base is used to attempt to bind
		let mut found_user = None;
		for search_base in &self.search_bases {
			let (rs, _res) = ldap
				.search(
					search_base.as_str(),
					Scope::Subtree,
					format!("({}={username})", self.user_identifier_attribute).as_str(),
					Vec::<&str>::new(),
				)
				.await
				.map_err(|_| "unable to attempt a search operation")?
				.success()
				.map_err(|_| "unable to find a user by the specified username")?;
			// Verify the result
			match rs.len().cmp(&1) {
				Ordering::Greater => return Ok(None),
				Ordering::Equal => {
					found_user = Some(SearchEntry::construct(rs[0].clone()));
					break;
				}
				Ordering::Less => continue,
			}
		}

		if let Some(user_entry) = found_user {
			// dbg!(&user_entry);

			// Attempt to bind to the found user with the provided password - this is what
			// actually does the authentication
			let success = ldap
				.simple_bind(user_entry.dn.as_str(), password)
				.await
				.map_err(|_| "unable to attempt a bind operation")?
				.success()
				.is_ok();

			// Unbind the handle
			ldap.unbind()
				.await
				.map_err(|_| "unable to unbind the handle")?;

			// Fetch the user display name
			let display_name = match &user_entry.attrs.get(&self.display_name_attribute) {
				Some(attribute_vec) if attribute_vec.len() == 1 => &attribute_vec[0],
				Some(_) => {
					// This *could* be an error, but that would mean the system would return an
					// internal error whenever someone tries to log in without a display name set.
					eprintln!("user has no display name: {username}");
					return Ok(None);
				}
				None => {
					return Err("unable to find the user display name with the provided attribute")
				}
			};

			// Return the result
			if success {
				Ok(Some(AuthenticationReturn {
					distinguished_name: user_entry.dn,
					unique_identifier: username.to_owned(),
					display_name: display_name.trim().to_owned(),
				}))
			} else {
				Ok(None)
			}
		} else {
			Ok(None)
		}
	}
}

impl From<AuthenticationReturn> for UserNew {
	fn from(ret: AuthenticationReturn) -> Self {
		Self {
			source: UserSource::Ldap,
			unique_identifier: ret.unique_identifier,
			display_name: ret.display_name,
		}
	}
}
