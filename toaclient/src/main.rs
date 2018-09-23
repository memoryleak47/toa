extern crate toalib;
extern crate sfml;
#[macro_use]
extern crate lazy_static;

mod cli;
#[macro_use]
mod index;
mod vec_compat;
mod graphics;
mod sound;
mod input;
mod controller;
mod view;

use sfml::window::{Style, Event};
use sfml::graphics::{RenderWindow, RenderTarget, Color};

use toalib::packet::{ServerToClientPacket, ClientToServerPacket};
use toalib::net::Stream;

use self::input::Input;
use self::graphics::TextureState;
use self::controller::Controller;
use self::sound::SoundState;

fn main() {
	let ip = cli::get_ip();

	let mut sound_state = SoundState::new().unwrap();
	sound_state.start();

	let mut stream = Stream::connect(&*ip);

	let (mut world, my_id) = match stream.receive_blocking() {
		ServerToClientPacket::Init { world, your_id } => (world, your_id),
		_ => panic!("got command packet while still in lobby!"),
	};

	let texture_state = TextureState::new();
	let mut controller = Controller::new(my_id);
	let mut input = Input::new();
	let mut window = RenderWindow::new((800, 600), "Toa client", Style::CLOSE, &Default::default());

	window.set_framerate_limit(60);

	while window.is_open() {
		while let Some(event) = window.poll_event() {
			if event == Event::Closed {
				window.close();
			}
		}

		window.set_active(true);

		match stream.receive_nonblocking() {
			Some(ServerToClientPacket::Command { author_id, command }) => assert!(world.checked_exec(author_id, &command)),
			Some(_) => panic!("got wrong packet while running!"),
			None => {},
		}

		input.tick();
		if let Some(c) = controller.tick(&world, &input) {
			let p = ClientToServerPacket::Command(c);
			stream.send(p);
		}

		controller.get_view(&world)
			.render(&mut window, &world, &texture_state);

		window.display();
		window.clear(&Color::rgb(0, 0, 0));
	}
}
