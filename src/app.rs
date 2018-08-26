use sfml::graphics::{RenderWindow, Color, RenderTarget};
use sfml::window::{Event::*, Style, VideoMode};

use input::Input;
use sound::SoundState;
use player::{Player, LocalPlayer, AiPlayer};
use world::World;
use command::Command;
use graphics::TextureState;

enum AppState {
	Menu,
	InGame {
		players: [Box<Player>; 2],
		world: World,
	}
}

pub struct App {
	window: RenderWindow,
	state: AppState,
	input: Input,
	texture_state: TextureState,
	sound_state: SoundState,
}

enum GameMode {
	LocalPvp,
	LocalVsAi,
}

fn get_players(mode: GameMode) -> [Box<Player>; 2] {
	match mode {
		GameMode::LocalPvp => [Box::new(LocalPlayer::new(0)), Box::new(LocalPlayer::new(1))],
		GameMode::LocalVsAi => [Box::new(LocalPlayer::new(0)), Box::new(AiPlayer::new(1))],
	}
}

impl App {
	pub fn new() -> App {
		let sound_state = match SoundState::new() { // TODO there definitely is a better way
			Ok(x) => x,
			Err(x) => panic!("SoundState::new() - Error: {}", x),
		};

		App {
			state: AppState::InGame {
				players: get_players(GameMode::LocalPvp),
				world: World::gen(),
			},
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Tales of Arbenhal", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			input: Input::new(),
			texture_state: TextureState::new(),
			sound_state,
		}
	}

	pub fn run(&mut self) {

		self.sound_state.start();
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
		if let AppState::InGame { ref mut world, ref mut players } = self.state {
			if let Some(command) = players[world.active_player as usize].tick(world, &self.input) {
				world.exec(&command);

				if let Command::NextTurn = command {
					players[1 - world.active_player as usize].turn_end();
					players[world.active_player as usize].turn_start();
				}
			}
		}
	}

	fn render(&mut self) {
		if let AppState::InGame { ref world, ref players, .. } = self.state {
			let view = players[world.active_player as usize].get_view(world);
			view.render(&mut self.window, world, &self.texture_state);
		}
	}
}
