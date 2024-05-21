use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use super::handlers::{
    global,
    // interv
};

use super::models::auth::{
    // AuthToken,
    AuthUser
};

use super::models::credentials::{
	// CredentialsJoin,
    CredentialsLogin,
    // CredentialsRefresh,
    CredentialsSignup,
};

use crate::app::providers::services::auth::claims::Claims;
use crate::app::providers::services::auth::db::DbAuth;
// use crate::app::providers::services::auth::token::Token;

pub fn routes() -> Vec<rocket::Route> {
	routes![options_all, signup, login, g_refresh,] // join, refresh, ]
}

#[options("/<_..>")]
pub async fn options_all() -> Status {
	Status::Ok
}

#[post("/signup", data = "<credentials>")]
async fn signup(
	db: &State<DbAuth>,
	credentials: Json<CredentialsSignup>,
) -> Result<Json<AuthUser>, Status> {
	let cred = credentials.into_inner();

	let response = global::signup(db, cred).await?;

	Ok(Json(response))
}

#[post("/login", data = "<credentials>")]
async fn login(
	db: &State<DbAuth>,
	credentials: Json<CredentialsLogin>,
) -> Result<Json<AuthUser>, Status> {
	// let mut cred = credentials.into_inner();
	let cred = credentials.into_inner();

	// if cred.username.contains("guest") {
	// 	// generate a guest user
	// 	let temp = global::generate_guest_user(&db).await?;

	// 	cred.username = temp.username;
	// 	cred.password = temp.password;
	// }

	let response = global::login(db, cred).await?;

	Ok(Json(response))
}

#[get("/refresh")]
async fn g_refresh(db: &State<DbAuth>, claims: Claims) -> Result<Json<AuthUser>, Status> {
	let user = global::get_auth_from_id(db, &claims.id).await?;

	Ok(Json(user))
}

// {{{
// #[post("/join", data = "<credentials>")]
// async fn join(
// 	db: &State<DbAuth>,
// 	claims: Claims,
// 	credentials: Json<CredentialsJoin>,
// ) -> Result<Json<AuthToken>, Status> {
// 	let mut cred = credentials.into_inner();

// 	// if cred.pass.contains("guest") {
// 	// 	cred.pass = interv::inject_guest(db, &claims.id, &cred).await?;
// 	// }

// 	let response = interv::join(db, claims, cred).await?;

// 	Ok(Json(response.into()))
// }

// #[post("/refresh", data = "<credentials>")]
// async fn refresh(
// 	db: &State<DbAuth>,
// 	claims: Claims,
// 	credentials: Json<CredentialsRefresh>,
// ) -> Result<Json<AuthToken>, Status> {
// 	let cred = credentials.into_inner();
// 	let token = Token(cred.token);
// 	let response;

// 	match cred.ns.as_ref() {
// 		"global" => {
// 			response = global::refresh_global_token(token)?;
// 			Ok(Json(response.into()))
// 		}
// 		// "interventions" => {
// 		// 	response = interv::refresh_interv_token(db, &cred.db, token).await?;
// 		// 	Ok(Json(response.into()))
// 		// }
// 		_ => {
// 			response = interv::refresh_interv_token(db, &cred.db, token).await?;
// 			Ok(Json(response.into()))
// 		}
// 	}
// }
// }}}
