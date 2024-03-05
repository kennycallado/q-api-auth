use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use super::claims::Claims;
use super::error::AuthError;
use super::token::Token;

use crate::app::providers::config::getter::ConfigGetter;

#[async_trait]
impl<'r> FromRequest<'r> for Claims {
	type Error = AuthError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let token = match Token::from_header(request) {
			Some(token) => token,
			None => return Outcome::Error((Status::Unauthorized, AuthError::MissingToken)),
		};

		let secret_key = ConfigGetter::get_secret_key();
		// let secret_key = match ConfigGetter::get_secret_key() {
		// 	None => panic!("secret_key is mandatory"),
		// 	Some(secret_key) => secret_key,
		// };

		let claims = match token.decode(secret_key.as_ref()) {
			Ok(claims) => claims.claims,
			Err(_) => return Outcome::Error((Status::Unauthorized, AuthError::InvalidToken)),
		};

		Outcome::Success(claims)
	}
}
