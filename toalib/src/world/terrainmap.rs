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
}

impl Terrain {
	pub fn get_stamina_cost(&self) -> u32 {
		match self {
			Terrain::GRASS => 0,
			Terrain::FOREST => 10,
			Terrain::STONE => 10,
			Terrain::IRON => 10,
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
}

pub fn new_terrainmap() -> Vec<Terrain> {
	let mut rng = thread_rng();

	let mut terrainmap = init2d!(Terrain::GRASS, MAP_SIZE_Y, MAP_SIZE_X);
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			let r = rng.next_u32();
			if r % 3 == 0 {
				terrainmap[index2d!(x, y)] = Terrain::FOREST;
			} else if r % 7 == 0 {
				terrainmap[index2d!(x, y)] = Terrain::STONE;
			} else if r % 11 == 0 {
				terrainmap[index2d!(x, y)] = Terrain::IRON;
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
