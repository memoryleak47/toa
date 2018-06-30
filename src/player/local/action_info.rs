use std::slice;

use sfml::window::Key;

use player::local::{LocalPlayer, UnitMode, Action};
use input::Input;
use world::{World, buildingmap::BuildingClass}; use world::buildingmap::farm::FARM_CLASS;
use world::buildingmap::BUILDABLE_CLASSES;
use command::Command;
use misc::Direction;

pub static KEYED_BUILDABLE_CLASSES: [(&BuildingClass, Key); 1] = [(&FARM_CLASS, Key::F)];

pub struct ActionInfo {
	pub text: String,
	pub action: Action,
	pub key_combination: &'static [Key],
	pub triggered: fn(&Input, &[Key]) -> bool,
}

mod trigger {
	use sfml::window::Key;
	use input::Input;

	type F = fn(&Input, &[Key]) -> bool;
	pub static FRESH: F = |i, k| {
		i.are_pressed(k)
		&&
		i.is_fresh_pressed(*k.last().unwrap())
	};
	pub static PERMANENT: F = |i, k| i.are_pressed(k);
	pub static MOD: F = |i, k| i.are_pressed_mod(k, 3);
}

impl LocalPlayer {
	fn get_general_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		assert!(KEYED_BUILDABLE_CLASSES.len() == BUILDABLE_CLASSES.len());

		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "next turn".to_string(),
			action: Action::Command(Command::NextTurn),
			key_combination: &[Key::N],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "next unit".to_string(),
			action: Action::NextUnit,
			key_combination: &[Key::U],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "work".to_string(),
			action: Action::Command(Command::Work { at: self.cursor }),
			key_combination: &[Key::Q],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "unrefined work".to_string(),
			action: Action::Command(Command::UnrefinedWork { at: self.cursor }),
			key_combination: &[Key::H],
			triggered: trigger::FRESH,
		});

		// move camera:
		v.push(ActionInfo {
			text: "move camera up".to_string(),
			action: Action::MoveCamera(Direction::Up),
			key_combination: &[Key::LControl, Key::W],
			triggered: trigger::PERMANENT,
		});
		v.push(ActionInfo {
			text: "move camera left".to_string(),
			action: Action::MoveCamera(Direction::Left),
			key_combination: &[Key::LControl, Key::A],
			triggered: trigger::PERMANENT,
		});
		v.push(ActionInfo {
			text: "move camera down".to_string(),
			action: Action::MoveCamera(Direction::Down),
			key_combination: &[Key::LControl, Key::S],
			triggered: trigger::PERMANENT,
		});
		v.push(ActionInfo {
			text: "move camera right".to_string(),
			action: Action::MoveCamera(Direction::Right),
			key_combination: &[Key::LControl, Key::D],
			triggered: trigger::PERMANENT,
		});


		v
	}

	fn get_normal_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "unfocus unit".to_string(),
			action: Action::ModeChange(None),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move up".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Up}),
			key_combination: &[Key::W],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move left".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Left}),
			key_combination: &[Key::A],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move down".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Down}),
			key_combination: &[Key::S],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move right".to_string(),
			action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Right}),
			key_combination: &[Key::D],
			triggered: trigger::MOD,
		});

		v.push(ActionInfo {
			text: "go to attack mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Attack { target_cursor: self.cursor })),
			key_combination: &[Key::F],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "go to build mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Build)),
			key_combination: &[Key::B],
			triggered: trigger::FRESH,
		});

		v
	}

	fn get_attack_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move target cursor up".to_string(),
			action: Action::MoveTargetCursor(Direction::Up),
			key_combination: &[Key::W],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor left".to_string(),
			action: Action::MoveTargetCursor(Direction::Left),
			key_combination: &[Key::A],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor down".to_string(),
			action: Action::MoveTargetCursor(Direction::Down),
			key_combination: &[Key::S],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor right".to_string(),
			action: Action::MoveTargetCursor(Direction::Right),
			key_combination: &[Key::D],
			triggered: trigger::MOD,
		});

		v
	}

	fn get_build_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});

		for (b, key) in KEYED_BUILDABLE_CLASSES.iter() {
			v.push(ActionInfo {
				text: format!("build {}", b.get_name()),
				action: Action::Command(Command::Build { class: *b, at: self.cursor }),
				key_combination: slice::from_ref(key),
				triggered: trigger::FRESH,
			});
		}

		v
	}

	fn get_no_mode_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "move cursor up".to_string(),
			action: Action::MoveCursor(Direction::Up),
			key_combination: &[Key::W],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move cursor left".to_string(),
			action: Action::MoveCursor(Direction::Left),
			key_combination: &[Key::A],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move cursor down".to_string(),
			action: Action::MoveCursor(Direction::Down),
			key_combination: &[Key::S],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move cursor right".to_string(),
			action: Action::MoveCursor(Direction::Right),
			key_combination: &[Key::D],
			triggered: trigger::MOD,
		});
		if w.get_unit(self.cursor)
				.filter(|x| x.owner == self.player_id)
				.is_some() {
			v.push(ActionInfo {
				text: "focus unit".to_string(),
				action: Action::ModeChange(Some(UnitMode::Normal)),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
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
		(self.triggered)(input, self.key_combination)
	}
}
