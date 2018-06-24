use sfml::system::Vector2u;

use misc::Direction;
use world::buildingmap::BuildingClass;

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	Attack { from: Vector2u, to: Vector2u },
	NextTurn,
	Build { at: Vector2u, class: &'static BuildingClass },
	Work { at: Vector2u }
}

impl Command {
	pub fn get_info_string(&self) -> String {
		match self {
			Command::Move { .. } => "[wasd]: move unit".to_owned(),
			Command::Attack { .. } => "[q]: go into attack mode".to_owned(),
			Command::NextTurn => "[n]: next turn".to_owned(),
			Command::Build { .. } => "[b]: go into build mode".to_owned(),
			Command::Work { .. } => "[e]: work".to_owned(),
		}
	}
}
