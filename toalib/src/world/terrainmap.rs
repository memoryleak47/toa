use rand::{RngCore, thread_rng};

use crate::vec::Pos;
use crate::tilemap::TileMap;
use crate::world::unitmap::Unit;
use crate::item::ItemClass;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
pub enum Terrain {
	GRASS,
	FOREST,
	STONE,
	IRON,
	MOUNTAIN,
	MARSH,
}

const SPAWN_DISTRIBUTION: [(Terrain, u32); 6] =
[
	(Terrain::GRASS, 30),
	(Terrain::FOREST, 15),
	(Terrain::STONE, 5),
	(Terrain::IRON, 3),
	(Terrain::MOUNTAIN, 10),
	(Terrain::MARSH, 10),
];

impl Terrain {
	pub fn get_stamina_cost(&self) -> u32 {
		match self {
			Terrain::GRASS => 20,
			Terrain::FOREST => 30,
			Terrain::STONE => 30,
			Terrain::IRON => 30,
			Terrain::MOUNTAIN => 50,
			Terrain::MARSH => 50,
		}
	}

	pub fn is_unrefined_workable(&self, _unit: &Unit) -> bool {
		match self {
			Terrain::GRASS | Terrain::FOREST => true,
			_ => false,
		}
	}

	pub fn get_item_class(&self) -> ItemClass {
		match self {
			Terrain::GRASS => ItemClass::Food,
			Terrain::FOREST => ItemClass::Wood,
			_ => panic!("get_item() can only be called on GRASS/FOREST"),
		}
	}

	pub fn is_blocking(&self) -> bool {
		match self {
			Terrain::MOUNTAIN => true,
			_ => false,
		}
	}

	pub fn prevents_building(&self) -> bool {
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
