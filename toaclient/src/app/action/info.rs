use std::slice;

use sfml::window::Key;

use toalib::world::buildingmap::{BUILDABLE_BUILDING_CLASSES, BuildingClass};
use toalib::world::World;
use toalib::item::{CRAFTABLE_ITEM_CLASSES, ItemClass, Inventory};
use toalib::command::{Command, UnitCommand};
use toalib::misc::Direction;

use crate::app::action::Action;
use crate::input::Input;
use crate::app::App;
use crate::unit_mode::{UnitMode, ItemUnitMode};

lazy_static! {
	pub static ref KEYED_BUILDABLE_CLASSES: [(BuildingClass, Key); 8] = [
		(BuildingClass::Farm, Key::F),
		(BuildingClass::Camp, Key::C),
		(BuildingClass::Sawmill, Key::S),
		(BuildingClass::StoneMine, Key::M),
		(BuildingClass::IronMine, Key::M),
		(BuildingClass::Workshop, Key::W),
		(BuildingClass::Castle, Key::D),
		(BuildingClass::StoneWall, Key::W),
	];
}

pub struct ActionInfo {
	pub text: String,
	pub action: Action,
	pub key_combination: &'static [Key],
	pub triggered: fn(&Input, &[Key]) -> bool,
}

mod trigger {
	use sfml::window::Key;
	use crate::input::Input;

	type F = fn(&Input, &[Key]) -> bool;
	pub static FRESH: F = |i, k| {
		i.are_pressed(k)
		&&
		i.is_fresh_pressed(*k.last().unwrap())
	};
	pub static PERMANENT: F = |i, k| i.are_pressed(k);
	pub static MOD: F = |i, k| i.are_pressed_mod(k, 3, 4);
}

impl ActionInfo {
	pub fn get_text(&self, w: &World) -> String {
		let v: Vec<_> = self.key_combination.iter()
			.map(|x| format!("{:?}", x))
			.collect();
		let key_string = v.join("+");
		let mut s = format!("[{}]: {}", key_string, self.text);
		if let Some(Command::UnitCommand { command: c, pos }) = self.action.get_command() {
			s = format!("{} ({})", s, c.get_stamina_cost(pos, w));
		}
		s
	}

	pub fn is_triggered(&self, input: &Input) -> bool {
		(self.triggered)(input, self.key_combination)
	}
}

impl App {
	pub fn get_action_infos(&self) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.extend(self.get_general_action_infos());

		match self.unit_mode {
			Some(UnitMode::Normal) => v.extend(self.get_normal_mode_action_infos()),
			Some(UnitMode::Attack { .. }) => v.extend(self.get_attack_mode_action_infos()),
			Some(UnitMode::Build) => v.extend(self.get_build_mode_action_infos()),
			Some(UnitMode::Item { iu_mode, index }) => v.extend(self.get_item_mode_action_infos(iu_mode, index)),
			Some(UnitMode::Craft { index }) => v.extend(self.get_crafting_mode_action_infos(index)),
			None => v.extend(self.get_no_mode_action_infos()),
		}

		v = v.into_iter()
			.filter(|x| x.action.is_valid(&self.world, self.player_id))
			.collect();

		v
	}

	fn get_general_action_infos(&self) -> Vec<ActionInfo> {
		assert!(KEYED_BUILDABLE_CLASSES.len() == BUILDABLE_BUILDING_CLASSES.len());

		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "next turn".to_string(),
			action: Action::RawCommand(Command::NextTurn),
			key_combination: &[Key::N],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "next unit".to_string(),
			action: Action::NextUnit,
			key_combination: &[Key::U],
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

		// zoom
		v.push(ActionInfo {
			text: "zoom in".to_string(),
			action: Action::ZoomIn,
			key_combination: &[Key::Num9],
			triggered: trigger::PERMANENT,
		});

		v.push(ActionInfo {
			text: "zoom out".to_string(),
			action: Action::ZoomOut,
			key_combination: &[Key::Num0],
			triggered: trigger::PERMANENT,
		});


		v
	}

	fn get_normal_mode_action_infos(&self) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "unfocus unit".to_string(),
			action: Action::ModeChange(None),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});

		// work
		v.push(ActionInfo {
			text: "work".to_string(),
			action: Action::RawCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::Work }),
			key_combination: &[Key::Q],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "unrefined work".to_string(),
			action: Action::RawCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::UnrefinedWork }),
			key_combination: &[Key::H],
			triggered: trigger::FRESH,
		});

		// burn
		v.push(ActionInfo {
			text: "burn building".to_string(),
			action: Action::RawCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::BurnBuilding }),
			key_combination: &[Key::I],
			triggered: trigger::FRESH,
		});

		// move
		v.push(ActionInfo {
			text: "move up".to_string(),
			action: Action::MoveUnit { direction: Direction::Up, pos: self.cursor },
			key_combination: &[Key::W],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move left".to_string(),
			action: Action::MoveUnit { direction: Direction::Left, pos: self.cursor },
			key_combination: &[Key::A],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move down".to_string(),
			action: Action::MoveUnit { direction: Direction::Down, pos: self.cursor },
			key_combination: &[Key::S],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move right".to_string(),
			action: Action::MoveUnit { direction: Direction::Right, pos: self.cursor },
			key_combination: &[Key::D],
			triggered: trigger::FRESH,
		});

		// change mode
		if let Some(u) = self.world.get_unit(self.cursor) {
			v.push(ActionInfo {
				text: "go to attack mode".to_string(),
				action: Action::ModeChange(Some(UnitMode::Attack { aim: u.aim() })),
				key_combination: &[Key::F],
				triggered: trigger::FRESH,
			});
		}

		v.push(ActionInfo {
			text: "go to build mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Build)),
			key_combination: &[Key::B],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "drop item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode: ItemUnitMode::Drop, index: 0 })),
			key_combination: &[Key::Z],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "take item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode: ItemUnitMode::Take, index: 0 })),
			key_combination: &[Key::T],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "exec item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode: ItemUnitMode::Exec, index: 0 })),
			key_combination: &[Key::R],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "craft".to_string(),
			action: Action::ModeChange(Some(UnitMode::Craft { index: 0 })),
			key_combination: &[Key::C],
			triggered: trigger::FRESH,
		});

		v.push(ActionInfo {
			text: "equip item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode: ItemUnitMode::ChangeMainItem, index: 0 })),
			key_combination: &[Key::E],
			triggered: trigger::FRESH,
		});

		v
	}

	fn get_attack_mode_action_infos(&self) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});
		v.push(ActionInfo {
			text: "move target cursor up".to_string(),
			action: Action::MoveAim(Direction::Up),
			key_combination: &[Key::W],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor left".to_string(),
			action: Action::MoveAim(Direction::Left),
			key_combination: &[Key::A],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor down".to_string(),
			action: Action::MoveAim(Direction::Down),
			key_combination: &[Key::S],
			triggered: trigger::MOD,
		});
		v.push(ActionInfo {
			text: "move target cursor right".to_string(),
			action: Action::MoveAim(Direction::Right),
			key_combination: &[Key::D],
			triggered: trigger::MOD,
		});
		if let Some(UnitMode::Attack { ref aim }) = self.unit_mode.as_ref() {
			v.push(ActionInfo {
				text: "attack".to_string(),
				action: Action::RawCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::Attack(aim.clone())}),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
			});
		} else { assert!(false); }

		v
	}

	fn get_build_mode_action_infos(&self) -> Vec<ActionInfo> {
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
				action: Action::RawCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::Build(*b)}),
				key_combination: slice::from_ref(key),
				triggered: trigger::FRESH,
			});
		}

		v
	}

	fn get_item_mode_action_infos(&self, iu_mode: ItemUnitMode, index: usize) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});

		if let ItemUnitMode::ChangeMainItem = iu_mode {
			v.push(ActionInfo {
				text: String::from("Unequip Item"),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::ChangeMainItem(None)}),
				key_combination: &[Key::Q],
				triggered: trigger::FRESH,
			});
		}

		let inv: &Inventory = match iu_mode { // TODO well... make this readable
			ItemUnitMode::Drop | ItemUnitMode::ChangeMainItem | ItemUnitMode::Exec => &(if let Some(u) = self.world.get_unit(self.cursor) { u } else { return v; }).inventory,
			ItemUnitMode::Take => &self.world.get_inventory(self.cursor),
		};

		let l = inv.iter().len();
		if l == 0 { return v; }

		// activate
		v.push(match iu_mode {
			ItemUnitMode::Drop => ActionInfo {
				text: format!("Drop Item {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::DropItem(index, None)}),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
			},
			ItemUnitMode::Take => ActionInfo {
				text: format!("Take Item {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::TakeItem(index)}),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
			},
			ItemUnitMode::ChangeMainItem => ActionInfo {
				text: format!("Choose Item {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::ChangeMainItem(Some(index))}),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
			},
			ItemUnitMode::Exec => ActionInfo {
				text: format!("Use Item {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::ExecItem(index)}),
				key_combination: &[Key::Return],
				triggered: trigger::FRESH,
			},
		});

		if let ItemUnitMode::Drop = iu_mode {
			v.push(ActionInfo {
				text: format!("Drop Item Left {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::DropItem(index, Some(Direction::Left))}),
				key_combination: &[Key::A],
				triggered: trigger::FRESH,
			});
			v.push(ActionInfo {
				text: format!("Drop Item Down {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::DropItem(index, Some(Direction::Down))}),
				key_combination: &[Key::S],
				triggered: trigger::FRESH,
			});
			v.push(ActionInfo {
				text: format!("Drop Item Right {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::DropItem(index, Some(Direction::Right))}),
				key_combination: &[Key::D],
				triggered: trigger::FRESH,
			});
			v.push(ActionInfo {
				text: format!("Drop Item Up {} ({})", inv.iter().nth(index).unwrap().get_class().get_name(), index),
				action: Action::BackCommand(Command::UnitCommand { pos: self.cursor, command: UnitCommand::DropItem(index, Some(Direction::Up))}),
				key_combination: &[Key::W],
				triggered: trigger::FRESH,
			});
		}

		// next
		let next_index = (index + 1) % l;
		v.push(ActionInfo {
			text: "next item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode, index: next_index })),
			key_combination: &[Key::O],
			triggered: trigger::FRESH,
		});

		// previous
		let prev_index = (index + l - 1) % l;
		v.push(ActionInfo {
			text: "previous item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Item { iu_mode, index: prev_index })),
			key_combination: &[Key::P],
			triggered: trigger::FRESH,
		});

		v
	}

	fn get_crafting_mode_action_infos(&self, index: usize) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "go to normal mode".to_string(),
			action: Action::ModeChange(Some(UnitMode::Normal)),
			key_combination: &[Key::Escape],
			triggered: trigger::FRESH,
		});

		let l = CRAFTABLE_ITEM_CLASSES.len();
		assert!(l > 0);
		let itemclass: ItemClass = CRAFTABLE_ITEM_CLASSES[index];

		// activate
		v.push(ActionInfo {
			text: format!("Craft Item {} ({})", itemclass.get_name(), index),
			action: Action::RawCommand(Command::UnitCommand { command: UnitCommand::Craft(itemclass), pos: self.cursor }),
			key_combination: &[Key::Return],
			triggered: trigger::FRESH,
		});

		// next
		let next_index = (index + 1) % l;
		v.push(ActionInfo {
			text: "next item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Craft { index: next_index })),
			key_combination: &[Key::O],
			triggered: trigger::FRESH,
		});

		// previous
		let prev_index = (index + l - 1) % l;
		v.push(ActionInfo {
			text: "previous item".to_string(),
			action: Action::ModeChange(Some(UnitMode::Craft { index: prev_index })),
			key_combination: &[Key::P],
			triggered: trigger::FRESH,
		});

		v
	}

	fn get_no_mode_action_infos(&self) -> Vec<ActionInfo> {
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
		if self.world.get_unit(self.cursor)
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
}
