use rand::{RngCore, thread_rng};

use crate::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
pub enum Terrain {
	GRASS,
	PLAINS,
	FOREST,
	STONE,
	IRON,
	MOUNTAIN,
	MARSH,
}

static SPAWN_DISTRIBUTION: &[(Terrain, u32)] =
&[
	(Terrain::GRASS, 30),
	(Terrain::PLAINS, 40),
	(Terrain::FOREST, 15),
	(Terrain::STONE, 5),
	(Terrain::IRON, 3),
	(Terrain::MOUNTAIN, 10),
	(Terrain::MARSH, 10),
];

impl Terrain {
	pub fn get_stamina_cost(self) -> u32 {
		10
	}

	pub fn str(self) -> &'static str {
		match self {
			Terrain::GRASS => "Grass",
			Terrain::PLAINS => "Plains",
			Terrain::FOREST => "Forest",
			Terrain::STONE => "Stone",
			Terrain::IRON => "Iron",
			Terrain::MOUNTAIN => "Mountain",
			Terrain::MARSH => "Marsh",
		}
	}

	pub fn terrain_work_stats(self, building: Option<&Building>) -> Option<(u32, ItemClass)> { // work-stamina-cost, item
		let class = building.map(|x| x.get_class());
		match (self, class) {
			(Terrain::GRASS, Some(BuildingClass::Farm))      => Some((40, ItemClass::Food)),
			(Terrain::FOREST, Some(BuildingClass::Sawmill))  => Some((40, ItemClass::Wood)),
			(Terrain::STONE, Some(BuildingClass::StoneMine)) => Some((40, ItemClass::Stone)),
			(Terrain::IRON, Some(BuildingClass::IronMine))   => Some((40, ItemClass::Iron)),

			(Terrain::GRASS, _)                              => Some((80, ItemClass::Food)),
			(Terrain::FOREST, _)                             => Some((80, ItemClass::Wood)),

			(Terrain::STONE, _)                              => None,
			(Terrain::IRON, _)                               => None,
			(Terrain::MOUNTAIN, _)                           => None,
			(Terrain::MARSH, _)                              => None,
			(Terrain::PLAINS, _)							 => None,
		}

	}

	pub fn is_blocking(self) -> bool {
		match self {
			Terrain::MOUNTAIN => true,
			_ => false,
		}
	}

	pub fn prevents_building(self) -> bool {
		match self {
			Terrain::MOUNTAIN | Terrain::MARSH => true,
			_ => false,
		}
	}
}

pub fn new_terrainmap() -> TileMap<Terrain> {
	let mut rng = thread_rng();

	let mut terrainmap = TileMap::new(Terrain::GRASS);

	let sum: u32 = SPAWN_DISTRIBUTION.iter().map(|x| x.1).sum();

	for p in Pos::iter_all() {
		let mut r: u32 = rng.next_u32() % sum;
		for &(t, n) in SPAWN_DISTRIBUTION.iter() {
			if r < n {
				terrainmap.set(p, t);
				break;
			} else {
				r -= n;
			}
		}
	}

	terrainmap
}
