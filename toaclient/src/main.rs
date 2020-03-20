extern crate toalib;
extern crate sfml;

mod misc;
mod config;
mod cli;
mod vec_compat;
mod graphics;
mod gameobject;
mod sound;
mod app;
mod menu;

use self::app::App;

fn main() {
	let ip = cli::get_ip();
	let mut app = App::connect(&ip);
	app.run();
}
