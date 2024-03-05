use crate::app::modules::routing as modules_routing;

use crate::app::providers::config::cors;
#[cfg(feature = "auth")]
use crate::app::providers::services::auth::db::DbAuth;

#[launch]
pub async fn rocket() -> _ {
	#[allow(unused_mut)]
	let mut rocket = rocket::build();

	#[cfg(feature = "auth")]
	{
		rocket = rocket.manage(DbAuth::new().await);
	}

	rocket.attach(cors::Cors).attach(system::router()).attach(modules_routing::router())
}

mod system {
	pub fn router() -> rocket::fairing::AdHoc {
		rocket::fairing::AdHoc::on_ignite("System Routes", |rocket| async {
			rocket.mount("/", routes![health])
		})
	}

	#[get("/health")]
	fn health() -> &'static str {
		"OK"
	}
}
