extern crate sfml;

mod world;
mod player;
mod app;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
