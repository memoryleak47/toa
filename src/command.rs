use sfml::system::Vector2u;
use misc::Direction;
use world::buildingmap::BuildingPlan;

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	NextTurn,
	Build { at: Vector2u, plan: &'static BuildingPlan<'static> }
}
