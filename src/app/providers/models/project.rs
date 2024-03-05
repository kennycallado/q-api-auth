use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Project {
	pub id: Thing,
	pub name: Cow<'static, str>,
}
