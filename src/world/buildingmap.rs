use std::ops::Deref;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::terrainmap::Terrain;
use item::ItemKind;

lazy_static! {
pub static ref BUILDING_PLANS: [BuildingPlan<'static>; 1] = [
		BuildingPlan {
			building: Box::new(Building {
				kind: BuildingKind::Farm { stamina: 0 },
				health: 100
			}),
			required_stamina: 100,
			required_terrain: Some(Terrain::GRASS),
			required_resources: &[],
		},
	];
}

impl Deref for Building {
	type Target = BuildingKind;

	fn deref(&self) -> &BuildingKind {
		&self.kind
	}
}

#[derive(Debug)]
pub struct BuildingPlan<'a> {
	pub building: Box<Building>,
	pub required_stamina: u32,
	pub required_terrain: Option<Terrain>,
	pub required_resources: &'a [ItemKind], // may contain the same element multiple times
}

#[derive(Debug, Clone)]
pub enum BuildingKind {
	InConstruction { required_stamina: u32, building: Box<Building> },
	Spawn { owner: u32, food: u32 },
	Farm { stamina: u32 },
	Sawmill { stamina: u32 },
	StoneMine { stamina: u32 },
	IronMine { stamina: u32 },
	Wall,
	HalfWall,
}

impl Building {
	pub fn work(&mut self) {
		match self {
			Building { ref health, kind: BuildingKind::InConstruction { ref required_stamina, ref building }} => {
				if *required_stamina > 10 {
					let kind = BuildingKind::InConstruction { required_stamina: *required_stamina - 10, building: building.clone() };
					*self = Building { kind, health: *health };
				} else {
					*self = (**building).clone();
				}
			},
			_ => {}
		}
	}
}

#[derive(Debug, Clone)]
pub struct Building {
	pub kind: BuildingKind,
	pub health: u32
}

impl BuildingKind {
	pub fn get_color(&self) -> Color {
		match self {
			BuildingKind::Spawn { owner, .. } =>  {
				if *owner == 0 {
					Color::rgba(255, 0, 0, 100)
				} else {
					Color::rgba(0, 0, 255, 100)
				}
			},
			BuildingKind::InConstruction { .. } => Color::rgb(100, 100, 100),
			_ => Color::rgb(50, 50, 50) // TODO
		}
	}

	pub fn is_burnable(&self) -> bool {
		match self {
			BuildingKind::Wall | BuildingKind::HalfWall | BuildingKind::Spawn { .. } => false,
			_ => true,
		}
	}

	pub fn is_workable(&self) -> bool {
		match self {
			BuildingKind::Wall | BuildingKind::HalfWall | BuildingKind::Spawn { .. } => false,
			_ => true,
		}
	}

	pub fn get_height(&self) -> u32 {
		match self {
			BuildingKind::Wall => 2,
			BuildingKind::HalfWall => 1,
			_ => 0,
		}
	}
}

pub fn new_buildingmap() -> [[Option<Building>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut buildingmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	buildingmap[MAP_SIZE_X / 2][0] = Some(Building { health: 100, kind: BuildingKind::Spawn { owner: 0, food: 0 }});
	buildingmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Building { health: 100, kind: BuildingKind::Spawn { owner: 1, food: 0 }});

	buildingmap
}

impl World {
	pub fn get_building(&self, p: Vector2u) -> Option<&Building> {
		self.buildingmap[p.x as usize][p.y as usize].as_ref()
	}

	pub fn get_building_mut(&mut self, p: Vector2u) -> Option<&mut Building> {
		self.buildingmap[p.x as usize][p.y as usize].as_mut()
	}
}
