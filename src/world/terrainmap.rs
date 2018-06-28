use sfml::system::Vector2u;
use rand::{RngCore, thread_rng};

use graphics::TextureId;
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::REQUIRED_UNREFINED_WORK_STAMINA;
use world::unitmap::Unit;
use item::{Item, ItemKind};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Terrain {
	GRASS,
	FOREST,
	STONE,
	IRON,
}

impl Terrain {
	pub fn get_texture_id(&self) -> TextureId {
		match self {
			Terrain::GRASS => TextureId::GrassTile,
			Terrain::FOREST => TextureId::ForestTile,
			Terrain::STONE => TextureId::StoneTile,
			Terrain::IRON => TextureId::IronTile,
		}
	}

	pub fn get_stamina_cost(&self) -> u32 {
		match self {
			Terrain::GRASS => 10,
			Terrain::FOREST => 20,
			Terrain::STONE => 20,
			Terrain::IRON => 20,
		}
	}

	pub fn is_unrefined_workable(&self, unit: &Unit) -> bool {
		(match self {
			Terrain::GRASS | Terrain::FOREST => true,
			_ => false,
		})
		&&
		(unit.stamina >= REQUIRED_UNREFINED_WORK_STAMINA)
	}

	pub fn get_item(&self) -> Item {
		match self {
			Terrain::GRASS => Item { kind: ItemKind::Food, health: 3 },
			Terrain::FOREST => Item { kind: ItemKind::Wood, health: 3 },
			_ => panic!("get_item() can only be called on GRASS/FOREST"),
		}
	}
}

pub fn new_terrainmap() -> [[Terrain; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut rng = thread_rng();

	let mut terrainmap = [[Terrain::GRASS; MAP_SIZE_Y]; MAP_SIZE_X];
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			let r = rng.next_u32();
			if r % 3 == 0 {
				terrainmap[x][y] = Terrain::FOREST;
			} else if r % 7 == 0 {
				terrainmap[x][y] = Terrain::STONE;
			} else if r % 11 == 0 {
				terrainmap[x][y] = Terrain::IRON;
			}
		}
	}

	terrainmap
}

impl World {
	pub fn get_terrain(&self, p: Vector2u) -> &Terrain {
		&self.terrainmap[p.x as usize][p.y as usize]
	}
}
