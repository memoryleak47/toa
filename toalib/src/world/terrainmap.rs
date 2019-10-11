use rand::{RngCore, thread_rng};

use crate::vec::Vec2u;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
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
			Terrain::GRASS => 0,
			Terrain::FOREST => 10,
			Terrain::STONE => 10,
			Terrain::IRON => 10,
			Terrain::MOUNTAIN => 20,
			Terrain::MARSH => 40,
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

pub fn new_terrainmap() -> Vec<Terrain> {
	let mut rng = thread_rng();

	let mut terrainmap = init2d!(Terrain::GRASS, MAP_SIZE_Y, MAP_SIZE_X);

	let sum: u32 = SPAWN_DISTRIBUTION.iter().map(|x| x.1).sum();

	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			let mut r: u32 = rng.next_u32() % sum;
			for &(t, n) in SPAWN_DISTRIBUTION.iter() {
				if r < n {
					terrainmap[index2d!(x, y)] = t;
					break;
				} else {
					r -= n;
				}
			}
		}
	}

	terrainmap
}

impl World {
	pub fn get_terrain(&self, p: Vec2u) -> &Terrain {
		&self.terrainmap[index2d!(p.x, p.y)]
	}
}
