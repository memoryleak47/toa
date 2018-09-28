use sfml::window::Style;
use sfml::graphics::RenderWindow;

use toalib::packet::ServerToClientPacket;
use toalib::net::Stream;
use toalib::vec::{Vec2u, Vec2f};

use crate::input::Input;
use crate::graphics::TextureState;
use crate::sound::SoundState;
use crate::app::App;

impl App {
	pub fn connect(ip: &str) -> App {
		let mut stream = Stream::connect(&*ip);

		let (world, my_id) = match stream.receive_blocking() {
			ServerToClientPacket::Init { world, your_id } => (world, your_id),
			_ => panic!("got command packet while still in lobby!"),
		};

		let mut app = App {
			player_id: my_id,
			unit_mode: None,
			focus_position: Vec2f::new(0., 0.),
			cursor: Vec2u::new(0, 0),
			pending: None,
			world,
			window: RenderWindow::new((800, 600), "Toa client", Style::CLOSE, &Default::default()),
			input: Input::new(),
			texture_state: TextureState::new(),
			sound_state: SoundState::new().unwrap(),
			stream,
		};

		app.init();

		app
	}

	fn init(&mut self) {
		self.window.set_framerate_limit(60);
		self.sound_state.start();
	}
}
