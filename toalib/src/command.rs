use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::misc::Direction;
use crate::world::World;
use crate::aim::Aim;
use crate::world::buildingmap::BuildingClass;

#[derive(Serialize, Deserialize, Clone)]
pub enum UnitCommand {
	Move(Direction),
	Attack(Aim),
	Build(BuildingClass),
	Work, // building-work
	UnrefinedWork, // terrain-work
	DropItem(usize, Option<Direction>),
	TakeItem(usize),
	BurnBuilding,
	Craft(ItemClass),
	ChangeMainItem(Option<usize>),
	ExecItem(usize),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
	UnitCommand { command: UnitCommand, pos: Vec2u },
	NextTurn,
}

impl UnitCommand {
	pub fn get_stamina_cost(&self, pos: Vec2u, w: &World) -> u32 {
		match self {
			UnitCommand::Move(dir) => {
				let to = dir.plus_vector(pos);
				let terrain_summand = (w.get_terrain(pos).get_stamina_cost() + w.get_terrain(to).get_stamina_cost()) / 2;
				let height_summand = 10 * (w.get_height(pos) as i32 - w.get_height(to) as i32).abs() as u32;
				let weight_summand = 2 * w.get_unit(pos).unwrap().get_weight() / 5;
				terrain_summand + height_summand + weight_summand
			},
			UnitCommand::Attack(_) => { 10 },
			UnitCommand::Build(_) => { 10 },
			UnitCommand::Work => { 40 },
			UnitCommand::UnrefinedWork => { 80 },
			UnitCommand::DropItem(_, _) => 0,
			UnitCommand::TakeItem(i) => {
				w.get_inventory(pos)
					.iter()
					.nth(*i)
					.unwrap()
					.get_class()
					.get_weight()
			},
			UnitCommand::BurnBuilding => 10,
			UnitCommand::Craft(_) => 10,
			UnitCommand::ChangeMainItem(_) => 0,
			UnitCommand::ExecItem(_) => 0,
		}
	}
}
