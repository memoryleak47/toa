use sfml::system::{Vector2u, Vector2i};

use world::{World, MAP_SIZE_X, MAP_SIZE_Y, buildingmap::BUILDING_PLANS};
use command::Command;
use misc::*;

impl World {
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

		// add NextTurn
		v.push(Command::NextTurn);

		// add Build
		for plan in BUILDING_PLANS.iter() {
			v.push(Command::Build { at: pos, plan });
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
				v.extend(self.get_commands_by_unit(player, Vector2u::new(x as u32, y as u32)));
			}
		}
		v
	}

	pub fn is_valid_command(&self, player: u32, command: &Command) -> bool {
	/*
		match command {
			Command::Move { from, direction } => { },
			Command::Attack { from, to } => { },
			Command::NextTurn => true,
			Command::Build { .. } => { },
			Command::Work { .. } => {}
		}
	*/
	true
	}
}
