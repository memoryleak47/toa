extern crate toalib;
extern crate sfml;
#[macro_use]
extern crate lazy_static;

mod cli;
#[macro_use]
mod index;
mod vec_compat;
mod graphics;
mod input;
mod local;
mod view;

use sfml::window::{Style, Event};
use sfml::graphics::RenderWindow;

use toalib::packet::{ServerToClientPacket, ClientToServerPacket};
use toalib::net::{Stream, NonBlockError};

use self::input::Input;
use self::graphics::TextureState;

fn main() {
	let ip = cli::get_ip();

	let mut stream = Stream::connect(&*ip).unwrap();

	let (mut world, my_id) = match stream.receive_blocking().unwrap() {
		ServerToClientPacket::Init { world, your_id } => (world, your_id),
		_ => panic!("got command packet while still in lobby!"),
	};

	let mut texture_state = TextureState::new();
	let mut player = local::LocalPlayer::new(my_id);
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
			Ok(ServerToClientPacket::Command { author_id, command }) => assert!(world.checked_exec(author_id, &command)),
			Ok(_) => panic!("got wrong packet while running!"),
			Err(NonBlockError::Empty) => {},
			Err(NonBlockError::Error(x)) => Err(x).unwrap(),
		}

		if let Some(c) = player.tick(&world, &input) {
			let p = ClientToServerPacket::Command(c);
			stream.send(p).unwrap();
		}

		player.get_view(&world)
			.render(&mut window, &world, &texture_state);

		window.display();
	}
}
