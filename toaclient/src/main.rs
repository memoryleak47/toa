extern crate toalib;
extern crate sfml;
#[macro_use]
extern crate lazy_static;
extern crate font_kit;

mod config;
mod font;
mod cli;
#[macro_use]
mod index;
mod vec_compat;
mod graphics;
mod sound;
mod input;
mod unit_mode;
mod app;

use self::app::App;

fn main() {
	let ip = cli::get_ip();
	let mut app = App::connect(&ip);
	app.run();
}
