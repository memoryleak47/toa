use sfml::system::Vector2u;

use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::buildingmap::BUILDABLE_CLASSES;
use crate::command::{Command, UnitCommand};
use crate::misc::*;

impl World {
	pub fn get_unitless_commands(&self, _player: u32) -> Vec<Command> {
		vec![Command::NextTurn]
	}

	#[allow(dead_code)]
	fn get_unchecked_commands_by_unit(&self, _player: u32, pos: Vector2u) -> Vec<Command> {
		let mut v = Vec::new();

		// add Move
		for d in [Direction::Left, Direction::Right, Direction::Up, Direction::Down].into_iter() {
			v.push(Command::UnitCommand { pos, command: UnitCommand::Move(*d) });
		}

		// add Attack
		// <this is still missing>

		// add Build
		for c in &BUILDABLE_CLASSES[..] {
			v.push(Command::UnitCommand { pos, command: UnitCommand::Build(*c) });
		}

		// add Work
		v.push(Command::UnitCommand { pos, command: UnitCommand::Work});

		v
	}

	#[allow(dead_code)]
	pub fn get_commands_by_unit(&self, player: u32, pos: Vector2u) -> Vec<Command> {
		self.get_unchecked_commands_by_unit(player, pos).into_iter()
			.filter(|x| self.is_valid_command(player, x))
			.collect()
	}

	#[allow(dead_code)]
	pub fn get_commands(&self, player: u32) -> Vec<Command> {
		let mut v = Vec::new();
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let pos = Vector2u::new(x as u32, y as u32);
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

	fn is_valid_unit_command(&self, player: u32, pos: Vector2u, command: &UnitCommand) -> bool {
		self.unitmap[pos.x as usize][pos.y as usize]
		.as_ref()
		.filter(|x| x.owner == player)
		.filter(|x| x.stamina > 0)
		.is_some()
		&&
		match command {
			UnitCommand::Move(direction) => {
				let to = direction.plus_vector(pos);

				self.get_unit(to).is_none()
				&& self.get_height(to).saturating_sub(self.get_height(pos)) != 2 // can't climb a wall!
				&& self.get_unit(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Attack(_to) => {
				// TODO in range-check

				self.get_unit(pos)
					.filter(|x| x.owner == player)
					.is_some()
			},
			UnitCommand::Build(class) => {
				let req_terrain = class.get_required_terrain();
				self.get_building(pos).is_none()
				&&
				self.get_unit(pos)
					.filter(|x| x.owner == player)
					.filter(|x| x.inventory.contains_all(class.get_build_item_cost()))
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
			UnitCommand::DropItem(i) => {
				self.get_unit(pos)
					.filter(|u| u.inventory.iter().len() > *i)
					.is_some()
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
				self.get_unit(pos)
					.filter(|x| x.owner == player)
					.filter(|x| x.inventory.contains_all(recipe))
					.is_some()
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

	pub fn is_valid_command(&self, player: u32, command: &Command) -> bool {
		match command {
			Command::NextTurn => true,
			Command::UnitCommand { ref command, pos } => self.is_valid_unit_command(player, *pos, command),
		}
	}
}
