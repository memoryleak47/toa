extern crate sfml;
extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod misc;
mod graphics;
mod item;
mod input;
mod command;
mod world;
mod player;
mod view;
mod app;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
