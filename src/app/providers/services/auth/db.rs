use std::sync::Arc;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::app::providers::config::getter::{ConfigGetter, DatabaseConfig};

pub struct DbAuth(pub Arc<Surreal<Client>>);
impl DbAuth {
	pub async fn new() -> Self {
		let config =
			DatabaseConfig::get_database_config("store").expect("Failed to obtain db config");

		let db = Surreal::new::<Ws>(format!("{}:{}", &config.host, config.port).as_str())
			.await
			.expect("Failed to connect to the database");

		db.signin(Root {
			username: &config.username,
			password: &config.password,
		})
		.await
		.expect("Failed to sign in");

		db.use_ns("global").use_db("main").await.expect("Failed to use the database");

		let secret_key = ConfigGetter::get_secret_key();
		let query =
			format!("DEFINE TOKEN user_scope ON SCOPE user TYPE HS256 VALUE '{}';", secret_key);
		db.query(query.as_str()).await.expect("Failed to set the token");

		DbAuth(Arc::new(db))
	}
}
