use crate::vec::Pos;
use crate::item::ItemClass;
use crate::vec::Direction;
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
	UnitCommand { command: UnitCommand, pos: Pos },
	NextTurn,
}

impl UnitCommand {
	// this function assumes that the command is valid
	pub fn get_stamina_cost(&self, pos: Pos, w: &World) -> u32 {
		match self {
			UnitCommand::Move(dir) => {
				let to = pos.map(|x| x + **dir).unwrap();
				let terrain_summand = (stamina_cost_at(pos, w) + stamina_cost_at(to, w)) / 2;
				let weight_summand = 2 * w.unitmap.get(pos).unwrap().get_weight() / 5;
				terrain_summand + weight_summand
			},
			UnitCommand::Attack(_) => { 10 },
			UnitCommand::Build(class) => {
				class.get_build_property().unwrap().stamina_cost
			},
			UnitCommand::Work => { 40 },
			UnitCommand::UnrefinedWork => { 80 },
			UnitCommand::DropItem(_, _) => 0,
			UnitCommand::TakeItem(i) => {
				w.itemmap.get(pos)
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

fn stamina_cost_at(pos: Pos, w: &World) -> u32 {
	w.buildingmap.get(pos).and_then(
			|x| x.get_class().reduces_walk_stamina()
		).unwrap_or_else(
			|| w.terrainmap.get(pos).get_stamina_cost()
		)
}
