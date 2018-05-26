extern crate sfml;
extern crate rand;

mod world;
mod player;
mod view;
mod app;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
