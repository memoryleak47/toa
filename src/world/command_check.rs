use sfml::system::{Vector2u, Vector2i};

use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::REQUIRED_UNREFINED_WORK_STAMINA;
use world::buildingmap::BUILDABLE_CLASSES;
use command::Command;
use misc::*;

impl World {
	pub fn get_unitless_commands(&self, player: u32) -> Vec<Command> {
		vec![Command::NextTurn]
	}

	fn get_unchecked_commands_by_unit(&self, player: u32, pos: Vector2u) -> Vec<Command> {
		let mut v = Vec::new();

		// add Move
		for d in [Direction::Left, Direction::Right, Direction::Up, Direction::Down].into_iter() {
			v.push(Command::Move { from: pos, direction: *d });
		}

		// add Attack
		const MAX_RANGE: i32 = 5;

		for rx in -MAX_RANGE..=MAX_RANGE {
			for ry in -MAX_RANGE..=MAX_RANGE {
				let target = vector_iu(vector_ui(pos) + Vector2i::new(rx, ry));
				v.push(Command::Attack { from: pos, to: target });
			}
		}

		for c in &BUILDABLE_CLASSES[..] {
			v.push(Command::Build { class: *c, at: pos });
		}

		// add Work
		v.push(Command::Work { at: pos });

		v
	}

	pub fn get_commands_by_unit(&self, player: u32, pos: Vector2u) -> Vec<Command> {
		self.get_unchecked_commands_by_unit(player, pos).into_iter()
			.filter(|x| self.is_valid_command(player, x))
			.collect()
	}

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

	pub fn is_valid_command(&self, player: u32, command: &Command) -> bool {
		match command {
			Command::Move { from, direction } => {
				let to = direction.plus_vector(*from);
				let stamina = self.required_walk_stamina(*from, *direction);

				self.get_unit(to).is_none()
				&& self.get_height(to).saturating_sub(self.get_height(*from)) != 2 // can't climb a wall!
				&& self.get_unit(*from)
					.filter(|x| x.owner == player)
					.filter(|x| x.stamina >= stamina)
					.is_some()
			},
			Command::Attack { from, to } => {
				// TODO in range-check

				let stamina = self.required_attack_stamina(*from, *to);
				self.get_unit(*from)
					.filter(|x| x.owner == player)
					.filter(|x| x.stamina >= stamina)
					.is_some()
			},
			Command::NextTurn => true,
			Command::Build { at, class } => {
				let req_terrain = class.get_required_terrain();
				self.get_building(*at).is_none()
				&&
				self.get_unit(*at)
					.filter(|x| x.owner == player)
					.filter(|x| x.inventory.contains_all(class.get_build_item_cost()))
					.is_some()
				&&
				(req_terrain.is_none() || req_terrain.as_ref() == Some(self.get_terrain(*at)))
			},
			Command::Work { at } => {
				self.get_building(*at)
					.filter(|b| {
						self.get_unit(*at)
							.filter(|u| u.owner == player)
							.filter(|u| b.is_workable(u))
							.is_some()
					})
					.is_some()
			},
			Command::UnrefinedWork { at } => {
				self.get_unit(*at)
					.filter(|u| u.stamina >= REQUIRED_UNREFINED_WORK_STAMINA)
					.filter(|u| self.get_terrain(*at)
						.is_unrefined_workable(u)
					).is_some()
			}
		}
	}
}
