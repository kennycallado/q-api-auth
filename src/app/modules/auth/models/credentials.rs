use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CredentialsLoging {
	pub username: Cow<'static, str>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CredentialsJoin {
	pub ns: Cow<'static, str>,
	pub db: Cow<'static, str>,
	pub pass: Cow<'static, str>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CredentialsRefresh {
	pub ns: Cow<'static, str>,
	pub db: Cow<'static, str>,
	pub token: Cow<'static, str>,
}
