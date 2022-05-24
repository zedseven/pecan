// Uses
use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};

/// The object that handles authentication with an LDAP server.
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
	/// The base to search in when searching a provided username.
	pub search_base: String,
	/// The attribute used to identify users. (what is searched-for when a user
	/// provides their username)
	///
	/// This varies depending on whether it's a plain LDAP server, or an Active
	/// Directory server. LDAP uses `uid`, but AD uses `sAMAccountName` (user
	/// identifier) or `userPrincipalName` (user identifier @ domain)
	pub user_identifier: String,
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
		self,
		username: &str,
		password: &str,
	) -> Result<Option<String>, &'static str> {
		// Validate the provided username - it can't contain certain special characters
		if username.contains(&['*', '+']) {
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

		// Search for the given username
		let (rs, _res) = ldap
			.search(
				self.search_base.as_str(),
				Scope::Subtree,
				format!("({}={})", self.user_identifier, username).as_str(),
				Vec::<&str>::new(),
			)
			.await
			.map_err(|_| "unable to attempt a search operation")?
			.success()
			.map_err(|_| "unable to find a user by the specified username")?;
		// Verify the result
		if rs.len() != 1 {
			return Ok(None);
		}
		let user_entry = SearchEntry::construct(rs[0].clone());
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

		// Return the result
		if success {
			Ok(Some(user_entry.dn))
		} else {
			Ok(None)
		}
	}
}
