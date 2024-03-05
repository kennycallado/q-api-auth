use std::borrow::Cow;

use rocket::serde::json::Value;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Role {
	Admin,
	Guest,
	User,
}

impl From<Role> for Cow<'static, str> {
	fn from(role: Role) -> Self {
		match role {
			Role::Admin => Cow::Borrowed("admin"),
			Role::Guest => Cow::Borrowed("guest"),
			Role::User => Cow::Borrowed("user"),
		}
	}
}

impl From<Cow<'static, str>> for Role {
	fn from(role: Cow<'static, str>) -> Self {
		match role.as_ref() {
			"admin" => Role::Admin,
			"guest" => Role::Guest,
			"user" => Role::User,
			_ => Role::User,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserGlobalPrev {
	pub id: Thing,
	pub project: Option<Thing>,
	pub username: Cow<'static, str>,
	pub role: Cow<'static, str>,
	pub web_token: Value,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserGlobal {
	pub id: Thing,
	pub project: Option<Thing>,
	pub username: Cow<'static, str>,
	pub role: Role,
	pub web_token: Value,
}

#[derive(Deserialize)]
pub struct UserIntervPrev {
	pub id: Thing,
	pub role: Cow<'static, str>,
	pub pass: Cow<'static, str>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInterv {
	pub id: Thing,
	pub role: Role,
	pub pass: Cow<'static, str>,
}
