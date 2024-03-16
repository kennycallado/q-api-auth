use std::borrow::Cow;

use rocket::http::Status;

use crate::app::modules::auth::models::credentials::CredentialsJoin;

use crate::app::providers::models::user::{UserInterv, UserIntervPrev};
use crate::app::providers::services::auth::claims::Claims;
use crate::app::providers::services::auth::db::DbAuth;
use crate::app::providers::services::auth::token::Token;

// needs a UserInterv struct
pub async fn inject_guest(
	db: &DbAuth,
	user_id: &Cow<'static, str>,
	cred: &CredentialsJoin,
) -> Result<Cow<'static, str>, Status> {
	let sql = format!(
		r#"
        USE NS {} DB {};
        LET $q_pass = rand::ulid();

        RETURN UPDATE {user_id} SET pass = $q_pass;
        RETURN <string> $q_pass;
        "#,
		cred.ns, cred.db,
	);

	let mut query = db.0.query(sql)
        .await
        .map_err(|_| {
            dbg!("Error querying");
            Status::InternalServerError
	    })?;

	let pass: Option<String> = query
		.take(query.num_statements() - 1)
		.map(|pass: Option<String>| pass)
		.map_err(|_| {
			dbg!("Error getting pass");
			Status::InternalServerError
		})?;

	match pass {
		Some(pass) => Ok(pass.into()),
		None => {
			dbg!("There was an error injecting guest user");
			Err(Status::InternalServerError)
		}
	}
}

pub async fn join(
	db: &DbAuth,
	claims: Claims,
	cred: CredentialsJoin,
) -> Result<Cow<'static, str>, Status> {
	let user = validate_pass(db, &cred).await?;
	let secret_key = get_project_token(db, &cred.db).await?;

	let user_id = user.id.to_string().into();
	if user_id != claims.id {
		dbg!("User id does not match token id");
		return Err(Status::InternalServerError);
	}

	let mut claims = Claims::new(
		cred.ns,
		cred.db,
		"user".into(),
		"user_scope".into(),
		user_id,
		claims.role,
	);

	match claims.encode_for_access(secret_key.as_ref()) {
		Ok(token) => Ok(token.into()),
		Err(_) => {
			dbg!("Error encoding token");
			Err(Status::InternalServerError)
		}
	}
}

pub async fn refresh_interv_token(
	db: &DbAuth,
	db_name: &Cow<'static, str>,
	token: Token,
) -> Result<Cow<'static, str>, Status> {
	let secret_key = get_project_token(db, db_name).await?;

	let mut claims = match token.decode(secret_key.as_ref()) {
		Ok(claims) => claims.claims,
		Err(_) => {
			dbg!("Error decoding token");
			return Err(Status::InternalServerError);
		}
	};

	match claims.encode_for_access(secret_key.as_ref()) {
		Ok(token) => Ok(token.into()),
		Err(_) => {
			dbg!("Error encoding token");
			Err(Status::InternalServerError)
		}
	}
}

/// Returns user id if pass
async fn validate_pass(db: &DbAuth, cred: &CredentialsJoin) -> Result<UserInterv, Status> {
	let sql = format!(
		r#"
        USE NS {} DB {};
        LET $q_user = (SELECT * FROM ONLY users WHERE crypto::argon2::compare(pass, '{}') LIMIT 1);

        RETURN $q_user;
        "#,
		cred.ns, cred.db, cred.pass,
	);

	let mut query = db.0.query(sql).await.map_err(|_| {
		dbg!("Error querying");
		Status::InternalServerError
	})?;

	let user: UserInterv = query
		.take(query.num_statements() - 1)
		.map(|user: Option<UserIntervPrev>| {
			let user = user.unwrap();
			UserInterv {
				id: user.id,
				pass: user.pass,
			}
		})
		.map_err(|_| {
			dbg!("Error getting user");
			Status::InternalServerError
		})?;

	Ok(user)
}

async fn get_project_token(
	db: &DbAuth,
	project_name: &Cow<'static, str>,
) -> Result<String, Status> {
	let mut query =
		db.0.query("RETURN SELECT VALUE token FROM ONLY projects WHERE name = $b_project LIMIT 1;")
		.bind(("b_project", project_name))
		.await
		.map_err(|_| {
			dbg!("Error querying");
			Status::InternalServerError
		})?;

	let project_token: Option<String> =
		query.take(query.num_statements() - 1).map_err(|_| {
			dbg!("Error getting token");
			Status::InternalServerError
		})?;

	match project_token {
		Some(secret_key) => Ok(secret_key),
		None => {
			dbg!("Error getting global token");
			Err(Status::InternalServerError)
		}
	}
}
