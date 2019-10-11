use crate::vec::Vec2u;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::buildingmap::{Building, BUILDABLE_BUILDING_CLASSES};
use crate::command::{Command, UnitCommand};
use crate::misc::*;
use crate::team::PlayerID;

impl World {
	pub fn get_unitless_commands(&self, _player: PlayerID) -> Vec<Command> {
		vec![Command::NextTurn]
	}

	#[allow(dead_code)]
	fn get_unchecked_commands_by_unit(&self, _player: PlayerID, pos: Vec2u) -> Vec<Command> {
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
	pub fn get_commands_by_unit(&self, player: PlayerID, pos: Vec2u) -> Vec<Command> {
		self.get_unchecked_commands_by_unit(player, pos).into_iter()
			.filter(|x| self.is_valid_command(player, x))
			.collect()
	}

	#[allow(dead_code)]
	pub fn get_commands(&self, player: PlayerID) -> Vec<Command> {
		let mut v = Vec::new();
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let pos = Vec2u::new(x as u32, y as u32);
				if self.get_unit(pos)
						.filter(|x| x.owner == player)
						.is_some() {
					v.extend(self.get_commands_by_unit(player, pos));
				}
			}
		}

		v.extend(self.get_unitless_commands(player));

		v
	}

	fn is_valid_unit_command(&self, player: PlayerID, pos: Vec2u, command: &UnitCommand) -> bool {
		self.unitmap[index2d!(pos.x, pos.y)]
		.as_ref()
		.filter(|x| x.owner == player)
		.filter(|x| x.stamina > 0)
		.is_some()
		&&
		match command {
			UnitCommand::Move(direction) => {
				let to = direction.plus_vector(pos);

				self.get_unit(to).is_none()
				&& self.allowed_to_go_to(pos, to)
				&& self.get_unit(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Attack(_aim) => {
				// TODO check that the aim is of the unit at pos

				self.get_unit(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Build(class) => {
				let prop = match class.get_build_property() {
					Some(x) => x,
					None => return false,
				};
				let req_terrain = prop.required_terrain;
				self.get_building(pos).is_none()
				&&
				self.get_unit(pos)
					.filter(|x| x.owner == player)
					.filter(|x| x.inventory.contains_all(prop.item_cost))
					.is_some()
				&&
				(req_terrain.is_none() || req_terrain.as_ref() == Some(self.get_terrain(pos)))
			},
			UnitCommand::Work => {
				self.get_building(pos)
					.filter(|b| b.is_workable(self, pos))
					.is_some()
			},
			UnitCommand::UnrefinedWork => {
				self.get_unit(pos)
					.filter(|u| self.get_terrain(pos)
						.is_unrefined_workable(u)
					).is_some()
			}
			UnitCommand::DropItem(i, dir) => {
				self.get_unit(pos)
					.filter(|u| u.inventory.iter().len() > *i)
					.is_some()
				&& match dir {
					Some(Direction::Left) => pos.x != 0,
					Some(Direction::Down) => pos.y != MAP_SIZE_Y as u32-1,
					Some(Direction::Right) => pos.x != MAP_SIZE_X as u32-1,
					Some(Direction::Up) => pos.y != 0,
					None => true,
				}
			},
			UnitCommand::TakeItem(i) => {
				self.get_inventory(pos)
					.iter()
					.len() > *i
			},
			UnitCommand::BurnBuilding => {
				self.get_building(pos)
					.filter(|x| x.is_burnable(self, pos))
					.is_some()
			},
			UnitCommand::Craft(class) => {
				let recipe = match class.get_recipe() { Some(x) => x, None => return false };
				if let Some(Building::Workshop(_)) = self.get_building(pos) {
					self.get_unit(pos)
						.filter(|x| x.owner == player)
						.filter(|x| x.inventory.contains_all(recipe))
						.is_some()
				} else { false }
			},
			UnitCommand::ChangeMainItem(opt_index) => {
				if let Some(i) = opt_index {
					self.get_unit(pos)
						.filter(|u| u.inventory.iter().len() > *i)
						.is_some()
				} else {
					self.get_unit(pos)
						.and_then(|x| x.main_item.as_ref())
						.is_some()
				}
			},
			UnitCommand::ExecItem(i) => {
				self.get_unit(pos)
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

	fn allowed_to_go_to(&self, from: Vec2u, to: Vec2u) -> bool {
		let player_id = self.get_unit(from).unwrap().owner;

		if self.get_terrain(to).is_blocking() {
			return false;
		}

		if self.get_building(to)
				.map(|b| b.is_blocking_against(player_id))
				.unwrap_or(false) {
			return false;
		}

		true
	}
}
