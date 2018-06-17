use sfml::window::Key;
use sfml::system::{Vector2f, Vector2u};

use player::Player;
use player::local::{LocalPlayer, UnitMode};
use view::{View, Marker, MarkerType, CURSOR_COLOR};
use input::Input;
use world::World;
use command::Command;
use misc::{Direction, vector_if};

pub enum Action {
	ModeChange(Option<UnitMode>),
	Command(Command),
	MoveCamera(Direction),
	MoveCursor(Direction),
	MoveTargetCursor(Direction),
	NextUnit,
}

pub struct ActionInfo {
	text: String,
	action: Action,
	key_combination: Vec<Key>,
	fresh: bool, // whether it requires fresh keys
}

impl LocalPlayer {
	fn get_general_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "next turn".to_string(),
			action: Action::Command(Command::NextTurn),
			key_combination: vec![Key::N],
			fresh: true,
		});

		v.push(ActionInfo {
			text: "next unit".to_string(),
			action: Action::NextUnit,
			key_combination: vec![Key::U],
			fresh: true,
		});

		// move camera:
		v.push(ActionInfo {
			text: "move camera up".to_string(),
			action: Action::MoveCamera(Direction::Up),
			key_combination: vec![Key::LControl, Key::W],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera left".to_string(),
			action: Action::MoveCamera(Direction::Left),
			key_combination: vec![Key::LControl, Key::A],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera down".to_string(),
			action: Action::MoveCamera(Direction::Down),
			key_combination: vec![Key::LControl, Key::S],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera right".to_string(),
			action: Action::MoveCamera(Direction::Right),
			key_combination: vec![Key::LControl, Key::D],
			fresh: false,
		});


		v
	}

	fn get_normal_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "unfocus unit".to_string(),
			action: Action::ModeChange(None),
			key_combination: vec![Key::Escape],
			fresh: true,
		});
		v.push(ActionInfo {
			text: "move up".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Up}),
			key_combination: vec![Key::W],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move left".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Left}),
			key_combination: vec![Key::A],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move down".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Down}),
			key_combination: vec![Key::S],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move right".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Right}),
			key_combination: vec![Key::D],
			fresh: false,
		});

		v.push(ActionInfo {
			text: "go to attack mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Attack { target_cursor: self.cursor })),
			key_combination: vec![Key::F],
			fresh: false,
		});

		v.push(ActionInfo {
			text: "go to build mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Build)),
			key_combination: vec![Key::B],
			fresh: false,
		});

		v
	}

	fn get_attack_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: vec![Key::Escape],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move target cursor up".to_string(),
			action: Action::MoveTargetCursor(Direction::Up),
			key_combination: vec![Key::W],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move target cursor left".to_string(),
			action: Action::MoveTargetCursor(Direction::Left),
			key_combination: vec![Key::A],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move target cursor down".to_string(),
			action: Action::MoveTargetCursor(Direction::Down),
			key_combination: vec![Key::S],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move target cursor right".to_string(),
			action: Action::MoveTargetCursor(Direction::Right),
			key_combination: vec![Key::D],
			fresh: false,
		});

		v
	}

	fn get_build_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: vec![Key::Escape],
			fresh: false,
		});

		v
	}

	fn get_no_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "move cursor up".to_string(),
			action: Action::MoveCursor(Direction::Up),
			key_combination: vec![Key::W],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move cursor left".to_string(),
			action: Action::MoveCursor(Direction::Left),
			key_combination: vec![Key::A],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move cursor down".to_string(),
			action: Action::MoveCursor(Direction::Down),
			key_combination: vec![Key::S],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move cursor right".to_string(),
			action: Action::MoveCursor(Direction::Right),
			key_combination: vec![Key::D],
			fresh: false,
		});
		if w.get_unit(self.cursor)
				.filter(|x| x.owner == self.player_id)
				.is_some() {
			v.push(ActionInfo {
				text: "focus unit".to_string(),
				action: Action::ModeChange(Some(UnitMode::Normal)),
				key_combination: vec![Key::Return],
				fresh: false,
			});
		}

		v
	}

	pub fn get_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.extend(self.get_general_action_infos(w));

		match self.unit_mode {
			Some(UnitMode::Normal) => v.extend(self.get_normal_mode_action_infos(w)),
			Some(UnitMode::Attack { .. }) => v.extend(self.get_attack_mode_action_infos(w)),
			Some(UnitMode::Build) => v.extend(self.get_build_mode_action_infos(w)),
			None => v.extend(self.get_no_mode_action_infos(w)),
		}

		v = v.into_iter()
			.filter(|x| x.is_valid(self.player_id, w))
			.collect();

		v
	}
}

impl ActionInfo {
	pub fn is_valid(&self, player_id: u32, w: &World) -> bool {
		if let Action::Command(ref c) = self.action {
			w.is_valid_command(player_id, c)
		} else {
			true
		}
	}

	pub fn get_text(&self) -> String {
		let v: Vec<_> = self.key_combination.iter()
			.map(|x| format!("{:?}", x))
			.collect();
		let key_string = v.join("+");
		format!("[{}]: {}", key_string, self.text)
	}

	pub fn is_triggered(&self, input: &Input) -> bool {
		if self.fresh {
			self.key_combination.iter()
				.all(|x| input.is_fresh_pressed(*x))
		} else {
			input.are_pressed_mod(&self.key_combination[..], 3)
		}
	}

	pub fn execute(self, player: &mut LocalPlayer, w: &World) -> Option<Command> {
		match self.action {
			Action::Command(c) => return Some(c),
			Action::NextUnit => {
				for x in w.find_next_unit_tile(player.cursor, player.player_id) {
					player.cursor = x;
				}
			}
			Action::ModeChange(m) => { player.unit_mode = m; },
			Action::MoveTargetCursor(d) => {
				if let Some(UnitMode::Attack { ref mut target_cursor }) = player.unit_mode.as_mut() {
					*target_cursor = d.plus_vector(*target_cursor);
				} else { assert!(false); }
			},
			Action::MoveCamera(d) => { player.focus_position = vector_if(d.to_vector()) / 2. + player.focus_position; },
			Action::MoveCursor(d) => { player.cursor = d.plus_vector(player.cursor); },
		}
		None
	}
}
