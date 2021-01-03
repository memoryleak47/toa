use crate::*;

// TODO by using usize for items I lose the advantages of the borrow checker!

#[derive(Serialize, Deserialize, Clone)]
pub enum UnitCommand {
	Move(Direction),
	Attack(Option<usize>, Vec2f), // relative mouse vector
	Build(BuildingClass),
	Work, // building-work, NOTE: currently unused
	TerrainWork,
	DropItem(usize, Option<Direction>),
	TakeItem(usize),
	BurnBuilding,
	Craft(ItemClass),
	ExecItem(usize), // NOTE: currently unused
	FarmFood,
	SpawnUnit(Direction),
	Idle, // consumes all your stamina
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
				let terrain_summand = stamina_cost_at(pos, w) + stamina_cost_at(to, w);
				let weight_summand = w.unitmap.get(pos).unwrap().get_weight();
				terrain_summand + weight_summand
			},
			UnitCommand::Attack(..) => { 10 },
			UnitCommand::Build(class) => {
				class.get_build_property().unwrap().stamina_cost
			},
			UnitCommand::Work => { 0 }, // there actually aren't any workable buildings currently
			UnitCommand::TerrainWork => {
				let t = w.terrainmap.get(pos);
				let b = w.buildingmap.get(pos);
				t.terrain_work_stats(b).unwrap().0
			},
			UnitCommand::DropItem(_, _) => 0,
			UnitCommand::TakeItem(i) => {
				w.itemmap.get(pos)
					.iter()
					.nth(*i)
					.unwrap()
					.get_class()
					.get_weight()
			},
			UnitCommand::BurnBuilding => 0,
			UnitCommand::Craft(_) => 0,
			UnitCommand::ExecItem(_) => 0,
			UnitCommand::FarmFood | UnitCommand::SpawnUnit(_) =>
				match w.buildingmap.get(pos).map(|b| b.get_class()) {
					Some(BuildingClass::Farm) => 40,
					_ => 80,
				},
			UnitCommand::Idle => 0,
		}
	}
}

pub fn stamina_cost_at(pos: Pos, w: &World) -> u32 {
	w.buildingmap.get(pos).and_then(
			|x| x.get_class().reduces_walk_stamina()
		).unwrap_or_else(
			|| w.terrainmap.get(pos).get_stamina_cost()
		)
}
