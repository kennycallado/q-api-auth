#[cfg(feature = "auth")]
use crate::app::modules::auth::controller::routes as auth_routes;

pub fn router() -> rocket::fairing::AdHoc {
	#[allow(unused_mut)]
	rocket::fairing::AdHoc::on_ignite("Modules Routes", |mut rocket| async {
		#[cfg(feature = "auth")]
		{
			rocket = rocket.mount("/auth", auth_routes());
		}

		rocket
	})
}
