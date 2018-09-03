use sfml::system::Vector2u;

use item::ItemClass;
use misc::Direction;
use world::World;
use world::buildingmap::BuildingClass;

pub enum UnitCommand {
	Move(Direction),
	Attack(Vector2u),
	Build(&'static BuildingClass),
	Work, // building-work
	UnrefinedWork, // terrain-work
	DropItem(usize),
	TakeItem(usize),
	BurnBuilding,
	Craft(&'static ItemClass),
}

pub enum Command {
	UnitCommand { command: UnitCommand, pos: Vector2u } ,
	NextTurn,
}

impl UnitCommand {
	pub fn get_stamina_cost(&self, pos: Vector2u, w: &World) -> u32 {
		match self {
			UnitCommand::Move(dir) => {
				let to = dir.plus_vector(pos);
				let terrain_summand = (w.get_terrain(pos).get_stamina_cost() + w.get_terrain(to).get_stamina_cost()) / 2;
				let height_summand = 10 * (w.get_height(pos) as i32 - w.get_height(to) as i32).abs() as u32;
				terrain_summand + height_summand
			},
			UnitCommand::Attack(_) => { 10 },
			UnitCommand::Build(_) => { 10 },
			UnitCommand::Work => { 40 },
			UnitCommand::UnrefinedWork => { 80 },
			UnitCommand::DropItem(_) => 10,
			UnitCommand::TakeItem(_) => 10,
			UnitCommand::BurnBuilding => 10,
			UnitCommand::Craft(_) => 10,
		}
	}
}
