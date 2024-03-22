use std::borrow::Cow;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
	pub host: Cow<'static, str>,
	pub port: u16,
	pub namespace: Cow<'static, str>,
	pub database: Cow<'static, str>,
	pub username: Cow<'static, str>,
	pub password: Cow<'static, str>,
}

impl DatabaseConfig {
	pub fn get_database_config(name: &str) -> Option<DatabaseConfig> {
		let name = format!("databases.{}", name);
		rocket::Config::figment().extract_inner::<DatabaseConfig>(name.as_str()).ok()
	}
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigGetter {
	pub origin_url: String,
	pub secret_key: String,
}

impl ConfigGetter {
	pub fn get_origin_url() -> String {
		rocket::Config::figment()
			.extract::<ConfigGetter>()
			.expect("Failed to get the origin url")
			.origin_url
	}

	pub fn get_secret_key() -> String {
		rocket::Config::figment()
			.extract::<ConfigGetter>()
			.expect("Failed to get the secret key")
			.secret_key
	}
}
