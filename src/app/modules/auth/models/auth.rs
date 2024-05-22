use std::borrow::Cow;

use rocket::serde::json::Value;
use rocket::serde::Serialize;

use crate::app::providers::models::project::Project;
// use crate::app::providers::models::user::UserGlobal;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
	pub id: Cow<'static, str>,
	pub role: Option<Cow<'static, str>>,
	pub project: Value,
	pub username: Cow<'static, str>,
	pub g_token: Cow<'static, str>,
	pub p_token: Option<Cow<'static, str>>,
}

// impl From<&UserGlobal> for AuthUser {
// 	fn from(user: &UserGlobal) -> Self {
// 		AuthUser {
// 			id: user.id.to_string().into(),
// 			role: user.role.clone().into(),
// 			project: user.project.clone().map(|p| p.to_string().into()).unwrap_or(Value::Null),
// 			username: user.username.to_owned(),
// 			g_token: "".into(),
// 			p_token: None,
// 		}
// 	}
// }

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectToSend {
	pub id: Cow<'static, str>,
	pub center: Option<Cow<'static, str>>,
	pub name: Cow<'static, str>,
}

impl From<Project> for ProjectToSend {
	fn from(project: Project) -> Self {
		ProjectToSend {
			id: project.id.to_string().into(),
			center: None,
			name: project.name,
		}
	}
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
	pub token: Cow<'static, str>,
}

impl From<Cow<'static, str>> for AuthToken {
	fn from(token: Cow<'static, str>) -> Self {
		AuthToken {
			token,
		}
	}
}

impl From<String> for AuthToken {
	fn from(token: String) -> Self {
		AuthToken {
			token: token.into(),
		}
	}
}
