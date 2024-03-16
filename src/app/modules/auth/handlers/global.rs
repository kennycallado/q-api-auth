use std::borrow::Cow;

use rocket::http::Status;
use rocket::serde::json;
use surrealdb::sql::Thing;

use crate::app::modules::auth::models::auth::{AuthUser, ProjectToSend};
use crate::app::modules::auth::models::credentials::{CredentialsSignin, CredentialsSignup};

use crate::app::providers::config::getter::ConfigGetter;

use crate::app::providers::models::project::Project;
use crate::app::providers::models::user::{Role, UserGlobal, UserGlobalPrev};

use crate::app::providers::services::auth::claims::Claims;
use crate::app::providers::services::auth::db::DbAuth;
use crate::app::providers::services::auth::token::Token;

pub async fn signup(db: &DbAuth, cred: CredentialsSignup) -> Result<AuthUser, Status> {
    let project: Option<Thing> = match cred.project {
        Some(project) => {
            let temp: Vec<&str> = project.split(":").collect();
            if temp.len() != 2 || temp[0].is_empty() || temp[1].is_empty() || temp[0] != "projects" {
                eprintln!("Bad project id");
                return Err(Status::BadRequest)
            }

            Some(Thing::from((temp[0], temp[1]))) // doesn't work well with numbers
        },
        None => None,
    };


    // create user
    let mut query =
        db.0.query(r#"
        LET $q_role = (SELECT VALUE id FROM ONLY roles WHERE name = 'parti' LIMIT 1);

        RETURN CREATE users CONTENT { username: $b_username, role: $q_role, project: $b_project };
        RETURN SELECT VALUE name from $q_role;
        "#)
        .bind(("b_username", &cred.username))
        .bind(("b_project", project))
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
            let role = role.clone();
            let user = user.expect("User not found");

            UserGlobal {
                id: user.id,
                project: user.project,
                username: user.username,
                role: role.into(),
                web_token: user.web_token,
            }
        }).map_err(|e| {
            let foo = surrealdb::Error::from(e).to_string();
            if foo.contains("users_username") { return Status::Conflict } // index unique

            eprintln!("Error getting user: {:?}", foo);
            dbg!("Error getting user");
            Status::InternalServerError
        })?;

    let mut user = AuthUser::from(&user);
    user.token = generate_global_token(&user.id, role)?;

    Ok(user)
}

pub async fn generate_guest_user(db: &DbAuth) -> Result<UserGlobal, Status> {
	let mut query =
		db.0.query(r#"
        LET $q_role = (SELECT * FROM ONLY roles WHERE name = 'guest' LIMIT 1);

        RETURN CREATE users CONTENT { username: rand::string(), role: $q_role.id };
        RETURN $q_role.name;
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

pub async fn login(db: &DbAuth, cred: CredentialsSignin) -> Result<AuthUser, Status> {
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
