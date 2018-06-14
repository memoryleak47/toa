use sfml::graphics::{RenderWindow, Color, RenderTarget};
use sfml::window::{Event::*, Style, VideoMode};

use input::Input;
use player::{Player, LocalPlayer};
use world::World;
use view::View;
use command::Command;
use graphics::TextureState;

enum AppState {
	Menu,
	InGame {
		players: [Box<Player>; 2],
		world: World,
		view: View,
	}
}

pub struct App {
	window: RenderWindow,
	state: AppState,
	input: Input,
	texture_state: TextureState,
}

impl App {
	pub fn new() -> App {
		App {
			state: AppState::InGame {
				players: [Box::new(LocalPlayer::new()), Box::new(LocalPlayer::new())],
				world: World::gen(),
				view: View::new(0),
			},
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Combat", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			input: Input::new(),
			texture_state: TextureState::new(),
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
		self.input.tick();
		if let AppState::InGame { ref mut world, ref mut players, ref mut view } = self.state {
			if let Some(command) = players[world.active_player as usize].tick(world, view, &self.input) {
				world.exec(&command, view);

				if let Command::NextTurn = command {
					// reset view
					*view = View::new(world.active_player);

					players[1 - world.active_player as usize].turn_end();
					players[world.active_player as usize].turn_start();
				}
			}
		}
	}

	fn render(&mut self) {
		if let AppState::InGame { ref world, ref view, .. } = self.state {
			view.render(&mut self.window, world, &self.texture_state);
		}
	}
}
