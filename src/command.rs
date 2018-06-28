use sfml::system::Vector2u;

use misc::Direction;
use world::buildingmap::BuildingClass;

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	Attack { from: Vector2u, to: Vector2u },
	NextTurn,
	Build { at: Vector2u, class: &'static BuildingClass },
	Work { at: Vector2u }, // building-work
	UnrefinedWork { at : Vector2u }, // terrain-work
}
