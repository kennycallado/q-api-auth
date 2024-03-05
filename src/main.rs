mod app;

#[macro_use]
extern crate rocket;
extern crate surrealdb;

fn main() {
	app::server::main();
}
