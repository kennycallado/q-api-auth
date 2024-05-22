use std::borrow::Cow;

use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
	pub ns: Cow<'static, str>,
	pub db: Cow<'static, str>,
	pub sc: Cow<'static, str>,
	pub tk: Cow<'static, str>,
	pub id: Cow<'static, str>,
	pub role: Option<Cow<'static, str>>,
	iat: i64,
	exp: i64,
}

impl Claims {
	pub fn new(
		ns: Cow<'static, str>,
		db: Cow<'static, str>,
		sc: Cow<'static, str>,
		tk: Cow<'static, str>,
		id: Cow<'static, str>,
		role: Option<Cow<'static, str>>,
	) -> Self {
		Self {
			ns,
			db,
			sc,
			tk,
			id,
			role,
			iat: 0,
			exp: 0,
		}
	}
	pub fn encode_for_access(&mut self, token: &[u8]) -> Result<String, Error> {
		let iat = chrono::Utc::now().timestamp();
		let exp = iat + 60 * 60 * 24; // 24 hours

		self.iat = iat;
		self.exp = exp;

		encode(&Header::new(Algorithm::HS256), &self, &EncodingKey::from_secret(token))
	}
}
