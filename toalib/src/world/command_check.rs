use crate::world::World;
use crate::world::buildingmap::{Building, BUILDABLE_BUILDING_CLASSES};
use crate::command::{Command, UnitCommand};

use crate::vec::*;
use crate::team::PlayerID;

impl World {
	pub fn get_unitless_commands(&self, _player: PlayerID) -> Vec<Command> {
		vec![Command::NextTurn]
	}

	#[allow(dead_code)]
	fn get_unchecked_commands_by_unit(&self, _player: PlayerID, pos: Pos) -> Vec<Command> {
		let mut v = Vec::new();

		// add Move
		for d in [Direction::Left, Direction::Right, Direction::Up, Direction::Down].into_iter() {
			v.push(Command::UnitCommand { pos, command: UnitCommand::Move(*d) });
		}

		// add Attack
		// <this is still missing>

		// add Build
		for c in &BUILDABLE_BUILDING_CLASSES[..] {
			v.push(Command::UnitCommand { pos, command: UnitCommand::Build(*c) });
		}

		// add Work
		v.push(Command::UnitCommand { pos, command: UnitCommand::Work});

		v
	}

	#[allow(dead_code)]
	pub fn get_commands_by_unit(&self, player: PlayerID, pos: Pos) -> Vec<Command> {
		self.get_unchecked_commands_by_unit(player, pos).into_iter()
			.filter(|x| self.is_valid_command(player, x))
			.collect()
	}

	#[allow(dead_code)]
	pub fn get_commands(&self, player: PlayerID) -> Vec<Command> {
		let mut v = Vec::new();
		for p in Pos::iter_all() {
			if self.unitmap.get(p)
					.filter(|x| x.owner == player)
					.is_some() {
				v.extend(self.get_commands_by_unit(player, p));
			}
		}

		v.extend(self.get_unitless_commands(player));

		v
	}

	fn is_valid_unit_command(&self, player: PlayerID, pos: Pos, command: &UnitCommand) -> bool {
		self.unitmap.get(pos)
		.filter(|x| x.owner == player)
		.filter(|x| x.stamina > 0)
		.is_some()
		&&
		match command {
			UnitCommand::Move(direction) => {
				let to = match pos.map(|x| x + **direction) {
					Some(x) => x,
					None => return false,
				};

				self.unitmap.get(to).is_none()
				&& self.allowed_to_go_to(pos, to)
				&& self.unitmap.get(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Attack(_aim) => {
				self.unitmap.get(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Build(class) => {
				let prop = match class.get_build_property() {
					Some(x) => x,
					None => return false,
				};
				let req_terrain = prop.required_terrain;
				self.buildingmap.get(pos).is_none()
				&&
				!self.terrainmap.get(pos).prevents_building()
				&&
				self.unitmap.get(pos)
					.filter(|x| x.owner == player)
					.filter(|x| x.inventory.contains_all(prop.item_cost))
					.is_some()
				&&
				(req_terrain.is_none() || req_terrain.as_ref() == Some(self.terrainmap.get(pos)))
			},
			UnitCommand::Work => {
				self.buildingmap.get(pos)
					.filter(|b| b.is_workable(self, pos))
					.is_some()
			},
			UnitCommand::UnrefinedWork => {
				self.unitmap.get(pos)
					.filter(|u| self.terrainmap.get(pos)
						.is_unrefined_workable(u)
					).is_some()
			}
			UnitCommand::DropItem(i, opt_dir) => {
				self.unitmap.get(pos)
					.filter(|u| u.inventory.iter().len() > *i)
					.is_some()
				&& opt_dir.map(|dir| {
					pos.map(|x| x + *dir).is_some()
				}).unwrap_or(true)
			},
			UnitCommand::TakeItem(i) => {
				self.itemmap.get(pos)
					.iter()
					.len() > *i
			},
			UnitCommand::BurnBuilding => {
				self.buildingmap.get(pos)
					.filter(|x| x.is_burnable(self, pos))
					.is_some()
			},
			UnitCommand::Craft(class) => {
				let recipe = match class.get_recipe() { Some(x) => x, None => return false };
				if let Some(Building::Workshop(_)) = self.buildingmap.get(pos) {
					self.unitmap.get(pos)
						.filter(|x| x.owner == player)
						.filter(|x| x.inventory.contains_all(recipe))
						.is_some()
				} else { false }
			},
			UnitCommand::ChangeMainItem(opt_index) => {
				if let Some(i) = opt_index {
					self.unitmap.get(pos)
						.filter(|u| u.inventory.iter().len() > *i)
						.is_some()
				} else {
					self.unitmap.get(pos)
						.and_then(|x| x.main_item.as_ref())
						.is_some()
				}
			},
			UnitCommand::ExecItem(i) => {
				self.unitmap.get(pos)
					.map(|u| u.inventory.iter())
					.and_then(|mut inv| inv.nth(*i))
					.filter(|x| x.is_execable(pos, self))
					.is_some()
			},
		}
	}

	pub fn is_valid_command(&self, player: PlayerID, command: &Command) -> bool {
		if !self.active_player_ids.contains(&player) { return false; }

		match command {
			Command::NextTurn => true,
			Command::UnitCommand { ref command, pos } => self.is_valid_unit_command(player, *pos, command),
		}
	}

	fn allowed_to_go_to(&self, from: Pos, to: Pos) -> bool {
		let player_id = self.unitmap.get(from).unwrap().owner;

		if self.terrainmap.get(to).is_blocking() {
			return false;
		}

		if self.buildingmap.get(to)
				.map(|b| b.is_blocking_against(player_id))
				.unwrap_or(false) {
			return false;
		}

		true
	}
}
