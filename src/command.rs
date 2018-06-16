use sfml::system::Vector2u;

use misc::Direction;
use world::buildingmap::BuildingPlan;

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	Attack { from: Vector2u, to: Vector2u },
	NextTurn,
	Build { at: Vector2u, plan: &'static BuildingPlan<'static> },
	Work { at: Vector2u }
}
