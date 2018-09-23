extern crate toalib;
extern crate sfml;
#[macro_use]
extern crate lazy_static;

mod net;
#[macro_use]
mod misc;
mod vec_compat;
mod graphics;
mod input;
mod local;
mod view;

use std::net::TcpStream;
use std::env;
use std::io::Read;

use sfml::window::{Style, Event};
use sfml::graphics::RenderWindow;

use toalib::packet::{ServerToClientPacket, ClientToServerPacket};

use self::net::{try_receiving_packet, send_packet};
use self::input::Input;
use self::graphics::TextureState;

fn main() {
	let args: Vec<_> = env::args()
			.collect();
	let str_args: Vec<_> = args.iter()
			.map(|x| &*x)
			.collect();
	let ip = match &str_args[..] {
		[_, x] => x,
		_ => panic!("invalid number of CLI parameters"),
	};

	let mut stream = TcpStream::connect(&*ip).unwrap();

	let mut init_string = String::new();
	stream.read_to_string(&mut init_string).unwrap();

	let (mut world, my_id) = match ServerToClientPacket::from_str(&*init_string).unwrap() {
		ServerToClientPacket::Init { world, your_id } => (world, your_id),
		_ => panic!("got command packet while still in lobby!"),
	};

	let mut texture_state = TextureState::new();
	let mut player = local::LocalPlayer::new(my_id);
	let mut input = Input::new();
	let mut window = RenderWindow::new((800, 600),
								 "Toa client",
								 Style::CLOSE,
								 &Default::default());
	window.set_framerate_limit(60);

	while window.is_open() {
		while let Some(event) = window.poll_event() {
			if event == Event::Closed {
				window.close();
			}
		}

		window.set_active(true);

		if let Some(x) = try_receiving_packet(&mut stream) {
			let (author_id, command) = match x.unwrap() {
				ServerToClientPacket::Command { author_id, command, } => (author_id, command),
				_ => panic!("got init packet while already running!"),
			};

			assert!(world.checked_exec(author_id, &command));
		}

		if let Some(c) = player.tick(&world, &input) {
			let p = ClientToServerPacket::Command(c);
			send_packet(p, &mut stream);
		}

		player.get_view(&world)
			.render(&mut window, &world, &texture_state);

		window.display();
	}
}
