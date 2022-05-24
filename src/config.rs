#[derive(Serialize, Deserialize)]
pub struct Config {
	pub ldap_settings: LdapSettings,
}

#[derive(Serialize, Deserialize)]
pub struct LdapSettings {
	pub enabled: bool,
}
