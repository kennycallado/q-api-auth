use std::borrow::Cow;

use rocket::serde::json::Value;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    Robot,
	Admin,
    Coord,
    Thera,
    Parti,
	Guest,
}

impl From<Role> for Cow<'static, str> {
	fn from(role: Role) -> Self {
		match role {
            Role::Robot => Cow::Borrowed("robot"),
			Role::Admin => Cow::Borrowed("admin"),
            Role::Coord => Cow::Borrowed("coord"),
            Role::Thera => Cow::Borrowed("thera"),
            Role::Parti => Cow::Borrowed("parti"),
            Role::Guest => Cow::Borrowed("guest"),
		}
	}
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_ref() {
            "robot" => Role::Robot,
            "admin" => Role::Admin,
            "coord" => Role::Coord,
            "thera" => Role::Thera,
            "parti" => Role::Parti,
            "guest" => Role::Guest,
            _ => Role::Parti,
        }
    }
}

impl From<Cow<'static, str>> for Role {
	fn from(role: Cow<'static, str>) -> Self {
		match role.as_ref() {
            "robot" => Role::Robot,
			"admin" => Role::Admin,
            "coord" => Role::Coord,
            "thera" => Role::Thera,
            "parti" => Role::Parti,
            "guest" => Role::Guest,
            _ => Role::Parti,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserGlobalPrev {
	pub id: Thing,
	pub username: Cow<'static, str>,
    pub password: Cow<'static, str>,
    pub role: Cow<'static, str>,
	pub project: Option<Thing>,
	pub web_token: Value,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserGlobal {
	pub id: Thing,
	pub username: Cow<'static, str>,
    pub password: Cow<'static, str>,
    pub role: Role,
	pub project: Option<Thing>,
	pub web_token: Value,
}

#[derive(Deserialize)]
pub struct UserIntervPrev {
	pub id: Thing,
	pub pass: Cow<'static, str>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInterv {
	pub id: Thing,
	pub pass: Cow<'static, str>,
}
