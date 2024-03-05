use std::borrow::Cow;

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use rocket::Request;

use super::claims::Claims;

pub struct Token(pub Cow<'static, str>);
impl Token {
	pub fn from_header(request: &Request<'_>) -> Option<Token> {
		let token = request.headers().get_one("Authorization")?;

		let token = token.replace("Bearer ", "");
		Some(Token(token.into()))
	}

	// pub fn from_cookie(request: &Request<'_>) -> Option<Token> {
	//     let token = request.cookies().get_private("refresh_token")?;

	//     request.cookies().remove_private(token.clone());

	//     let token = token.value().to_string();
	//     Some(Token(token))
	// }

	pub fn decode(&self, secret_key: &[u8]) -> Result<TokenData<Claims>, Error> {
		decode::<Claims>(&self.0, &DecodingKey::from_secret(secret_key), &Validation::default())
	}
}
