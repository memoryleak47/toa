use sfml::graphics::{RenderWindow, Color, RenderTarget};
use sfml::window::{Event::*, Style, VideoMode};

use player::{Player, LocalPlayer};
use world::World;
use view::View;

enum AppState {
	Menu,
	InGame {
		players: [Box<Player>; 2],
		world: World,
		view: View
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
				view: View::new(),
			},
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Combat", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
		}
	}

	pub fn run(&mut self) {

		self.window.set_framerate_limit(60);

		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				match event {
					Closed => self.window.close(),
					_ => {},
				}
			}

			self.tick();

			self.window.clear(&Color::rgb(0, 0, 0));

			self.render();
			self.window.display();

		}
	}

	fn tick(&mut self) {
		if let AppState::InGame { ref mut world, ref players, ref mut view } = self.state {
			world.tick(players, view);
		}
	}

	fn render(&mut self) {
		if let AppState::InGame { ref world, ref view, .. } = self.state {
			world.render(&mut self.window, view);
		}
	}
}
