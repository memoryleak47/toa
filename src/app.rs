use sfml::graphics::{RenderWindow, Color, RenderTarget};
use sfml::window::{Event::*, Style};

use player::{Player, LocalPlayer};
use world::World;

enum AppState {
	Menu,
	InGame {
		players: [Box<Player>; 2],
		world: World
	}
}

pub struct App {
	window: RenderWindow,
	state: AppState,
}

impl App {
	pub fn new() -> App {
		App {
			state: AppState::InGame {
				players: [Box::new(LocalPlayer::new()), Box::new(LocalPlayer::new())],
				world: World::gen(),
			},
			window: RenderWindow::new((800, 600), "Combat", Style::CLOSE, &Default::default()),
		}
	}

	pub fn run(&mut self) {
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				match event {
					Closed => self.window.close(),
					_ => {},
				}
			}
			self.window.clear(&Color::rgb(0, 0, 0));

			self.render();
			self.window.display();

		}
	}

	fn render(&mut self) {
		if let AppState::InGame { ref world, .. } = self.state {
			world.render(&mut self.window);
		}
	}
}
