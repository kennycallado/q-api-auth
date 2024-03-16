use std::borrow::Cow;

use rocket::http::Status;
use rocket::serde::json;

use crate::app::modules::auth::models::auth::{AuthUser, ProjectToSend};
use crate::app::modules::auth::models::credentials::CredentialsLoging;

use crate::app::providers::config::getter::ConfigGetter;

use crate::app::providers::models::project::Project;
use crate::app::providers::models::user::{Role, UserGlobal, UserGlobalPrev};

use crate::app::providers::services::auth::claims::Claims;
use crate::app::providers::services::auth::db::DbAuth;
use crate::app::providers::services::auth::token::Token;

pub async fn generate_guest_user(db: &DbAuth) -> Result<UserGlobal, Status> {
	let mut query =
		db.0.query(r#"
        LET $q_user = CREATE users CONTENT { username: rand::string(), role: roles:5 };

        RETURN $q_user;
        RETURN SELECT VALUE name from $q_user.role;
        "#)
        .await
        .map_err(|_| {
            dbg!("Error creating user");
            Status::InternalServerError
        })?;

    let role: Role = query
        .take(query.num_statements() - 1)
        .map(|role: Option<String>| {
            let role = role.unwrap();

            role.into()
        })
        .map_err(|_| {
        dbg!("Error getting role");
        Status::InternalServerError
    })?;

	let user: UserGlobal = query
		.take(query.num_statements() - 1)
		.map(|user: Option<UserGlobalPrev>| {
			let user = user.unwrap();
			UserGlobal {
				id: user.id,
				project: user.project,
				username: user.username,
				role: role.into(),
				web_token: user.web_token,
			}
		})
		.map_err(|_| {
			dbg!("Error getting user");
			Status::InternalServerError
		})?;

	Ok(user)
}

pub async fn login(db: &DbAuth, cred: CredentialsLoging) -> Result<AuthUser, Status> {
	let (mut user_to_send, role) = get_user_from_username(db, &cred.username).await?;
	user_to_send.token = generate_global_token(&user_to_send.id, role)?;

	Ok(user_to_send)
}

pub fn refresh_global_token(token: Token) -> Result<Cow<'static, str>, Status> {
	let secret_key = ConfigGetter::get_secret_key();
	// let secret_key = match ConfigGetter::get_secret_key() {
	// 	Some(secret_key) => secret_key,
	// 	None => {
	// 		dbg!("Error getting global token");
	// 		return Err(Status::InternalServerError);
	// 	}
	// };

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

fn generate_global_token(
	user_id: &Cow<'static, str>,
	role: Role,
) -> Result<Cow<'static, str>, Status> {
	// check if user is admin

	let mut claims = Claims::new(
		"global".into(),
		"main".into(),
		"user".into(),
		"user_scope".into(), // admin_scope
		user_id.to_string().into(),
		role.into(),
	);

	let secret_key = ConfigGetter::get_secret_key();
	// if let None = secret_key {
	// 	dbg!("Error getting global token");
	// 	return Err(Status::InternalServerError);
	// }

	match claims.encode_for_access(secret_key.as_bytes()) {
		Ok(token) => Ok(token.into()),
		Err(_) => {
			dbg!("Error encoding token");
			Err(Status::InternalServerError)
		}
	}
}

pub async fn get_user_from_username(
	db: &DbAuth,
	username: &Cow<'static, str>,
) -> Result<(AuthUser, Role), Status> {
	let mut query =
		db.0.query(
			r#"
            LET $q_user = (SELECT * FROM ONLY users WHERE username = $b_username LIMIT 1);

            RETURN SELECT * FROM ONLY $q_user.project LIMIT 1;
            RETURN $q_user;
            RETURN SELECT VALUE name from $q_user.role;
            "#,
		)
		.bind(("b_username", username))
		.await
		.map_err(|_| {
			dbg!("Error querying user");
			Status::InternalServerError
		})?;

    let role: Role = query
        .take(query.num_statements() - 1)
        .map(|role: Option<String>| {
            let role = role.unwrap();

            role.into()
        })
        .map_err(|_| {
        dbg!("Error getting role");
        Status::InternalServerError
    })?;

	let user: UserGlobal = query
		.take(query.num_statements() - 1)
		.map(|user: Option<UserGlobalPrev>| {
			let user = user.unwrap();
			UserGlobal {
				id: user.id,
				project: user.project,
				username: user.username,
				role: role.into(),
				web_token: user.web_token,
			}
		})
		.map_err(|_| {
			dbg!("Error getting user");
			Status::InternalServerError
		})?;

	let project: Option<Project> = query.take(query.num_statements() - 1).map_err(|_| {
		dbg!("Error getting project");
		Status::InternalServerError
	})?;

	let mut auth_user = AuthUser::from(&user);

	if let Some(project) = project {
		auth_user.project = json::to_value(ProjectToSend::from(project)).unwrap();
	}

	Ok((auth_user, user.role))

	// match user {
	// 	Some(user) => {
	// 		let mut auth_user = AuthUser::from(&user);

	// 		if let Some(project) = project {
	// 			auth_user.project = json::to_value(ProjectToSend::from(project)).unwrap();
	// 		}

	// 		Ok(auth_user)
	// 	}
	// 	None => {
	// 		dbg!("User not found");
	// 		Err(Status::NotFound)
	// 	}
	// }
}
