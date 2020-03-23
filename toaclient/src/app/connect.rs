use sfml::window::{Style, VideoMode};
use sfml::graphics::{RenderWindow, Font};

use toalib::packet::ServerToClientPacket;
use toalib::net::Stream;
use toalib::vec::{Pos, Vec2f};

use crate::misc::resource;
use crate::graphics::TextureState;
use crate::sound::SoundState;
use crate::app::App;
use crate::config::DEFAULT_TILESIZE;
use crate::menu::MenuState;

impl App {
	pub fn connect(ip: &str) -> App {
		let mut stream = Stream::connect(&*ip);

		let (world, my_id) = match stream.receive_blocking() {
			ServerToClientPacket::Init { world, your_id } => (world, your_id),
			_ => panic!("got command packet while still in lobby!"),
		};

		let mut app = App {
			player_id: my_id,
			focus_position: Vec2f::new(0., 0.),
			tilesize: DEFAULT_TILESIZE,
			cursor: Pos::build(0, 0).unwrap(),
			pending: vec![],
			menu_state: MenuState::new(),
			world,
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Toa client", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			texture_state: TextureState::new(),
			sound_state: SoundState::new().unwrap(),
			stream,
			font: Font::from_file(&resource("font/Monospace.ttf")).unwrap(),
			window_grab: None,
		};

		app.init();

		app
	}

	fn init(&mut self) {
		self.window.set_framerate_limit(60);
		self.sound_state.start();
	}
}
