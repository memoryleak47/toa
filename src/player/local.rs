use sfml::window::Key;
use sfml::system::Vector2f;

use player::Player;
use view::View;
use input::Input;
use world::{buildingmap::BUILDING_PLANS, World};
use command::Command;
use misc::{Direction, vector_if};

enum UnitMode {
	Normal,
	Attack,
	Build
}

enum Action {
	ModeChange(Option<UnitMode>),
	Command(Command),
	MoveCamera(Direction),
	MoveCursor(Direction),
}

struct ActionInfo {
	text: String,
	action: Action,
	key_combination: Vec<Key>,
}

pub struct LocalPlayer {
	unit_mode: Option<UnitMode>, // None -> no unit focused
}

impl LocalPlayer {
	pub fn new() -> LocalPlayer {
		LocalPlayer { unit_mode: None }
	}

	fn get_action_infos(&self, w: &World, view: &View) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "next turn".to_string(),
			action: Action::Command(Command::NextTurn),
			key_combination: vec![Key::N],
		});

		// move camera:
		v.push(ActionInfo {
			text: "move camera up".to_string(),
			action: Action::MoveCamera(Direction::Up),
			key_combination: vec![Key::LControl, Key::W]
		});
		v.push(ActionInfo {
			text: "move camera left".to_string(),
			action: Action::MoveCamera(Direction::Left),
			key_combination: vec![Key::LControl, Key::A]
		});
		v.push(ActionInfo {
			text: "move camera down".to_string(),
			action: Action::MoveCamera(Direction::Down),
			key_combination: vec![Key::LControl, Key::S]
		});
		v.push(ActionInfo {
			text: "move camera right".to_string(),
			action: Action::MoveCamera(Direction::Right),
			key_combination: vec![Key::LControl, Key::D]
		});

		match self.unit_mode {
			Some(UnitMode::Normal) => {
				v.push(ActionInfo {
					text: "unfocus unit".to_string(),
					action: Action::ModeChange(None),
					key_combination: vec![Key::Escape]
				});
				v.push(ActionInfo {
					text: "move up".to_string(),
					action: Action::Command(Command::Move { from: view.main_cursor, direction: Direction::Up}),
					key_combination: vec![Key::W]
				});
				v.push(ActionInfo {
					text: "move left".to_string(),
					action: Action::Command(Command::Move { from: view.main_cursor, direction: Direction::Left}),
					key_combination: vec![Key::A]
				});
				v.push(ActionInfo {
					text: "move down".to_string(),
					action: Action::Command(Command::Move { from: view.main_cursor, direction: Direction::Down}),
					key_combination: vec![Key::S]
				});
				v.push(ActionInfo {
					text: "move right".to_string(),
					action: Action::Command(Command::Move { from: view.main_cursor, direction: Direction::Right}),
					key_combination: vec![Key::D]
				});

				v.push(ActionInfo {
					text: "go to attack mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Attack)),
					key_combination: vec![Key::F]
				});

				v.push(ActionInfo {
					text: "go to build mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Build)),
					key_combination: vec![Key::B]
				});
			},
			Some(UnitMode::Attack) => {
				v.push(ActionInfo {
					text: "go to normal mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Normal)),
					key_combination: vec![Key::Escape]
				});
			}
			Some(UnitMode::Build) => {
				v.push(ActionInfo {
					text: "go to normal mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Normal)),
					key_combination: vec![Key::Escape]
				});
			}
			None => {
				v.push(ActionInfo {
					text: "move cursor up".to_string(),
					action: Action::MoveCursor(Direction::Up),
					key_combination: vec![Key::W]
				});
				v.push(ActionInfo {
					text: "move cursor left".to_string(),
					action: Action::MoveCursor(Direction::Left),
					key_combination: vec![Key::A]
				});
				v.push(ActionInfo {
					text: "move cursor down".to_string(),
					action: Action::MoveCursor(Direction::Down),
					key_combination: vec![Key::S]
				});
				v.push(ActionInfo {
					text: "move cursor right".to_string(),
					action: Action::MoveCursor(Direction::Right),
					key_combination: vec![Key::D]
				});
				if w.get_unit(view.main_cursor)
						.filter(|x| x.owner == w.active_player)
						.is_some() {
					v.push(ActionInfo {
						text: "focus unit".to_string(),
						action: Action::ModeChange(Some(UnitMode::Normal)),
						key_combination: vec![Key::Return]
					});
				}
			}
		}

		v = v.into_iter()
			.filter(|x| x.is_valid(w))
			.collect();

		v
	}
}

impl Player for LocalPlayer {
	fn tick(&mut self, w: &World, view: &mut View, input: &Input) -> Option<Command> {
		let action_infos = self.get_action_infos(w, view);

		let v: Vec<_> = action_infos.iter()
				.map(|x| x.get_text())
				.collect();
		view.text = v.join("\n");

		for info in action_infos.into_iter() {
			if info.is_triggered(input) {
				return info.execute(self, view);
			}
		}
		None
	}

	fn turn_start(&mut self) {
		self.unit_mode = None;
	}
}

impl ActionInfo {
	fn is_valid(&self, w: &World) -> bool {
		if let Action::Command(ref c) = self.action {
			w.is_valid_command(w.active_player, c)
		} else {
			true
		}
	}

	fn get_text(&self) -> String {
		let v: Vec<_> = self.key_combination.iter()
			.map(|x| format!("{:?}", x))
			.collect();
		let key_string = v.join("+");
		format!("[{}]: {}", key_string, self.text)
	}

	fn is_triggered(&self, input: &Input) -> bool {
		self.key_combination.iter()
			.all(|x| input.is_fresh_pressed(*x))
	}

	fn execute(self, player: &mut LocalPlayer, view: &mut View) -> Option<Command> {
		match self.action {
			Action::Command(c) => return Some(c),
			Action::ModeChange(m) => { player.unit_mode = m; },
			Action::MoveCamera(d) => { view.focus_position = vector_if(d.to_vector()) / 10. + view.focus_position; },
			Action::MoveCursor(d) => { view.main_cursor = d.plus_vector(view.main_cursor); },
		}
		None
	}
}
