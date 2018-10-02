use std::ptr;
use std::mem;

pub mod terrain;
pub mod building;

use sfml::graphics::Texture;

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum TextureId {
	GrassTerrain,
	ForestTerrain,
	StoneTerrain,
	IronTerrain,

	Unit,
	UnitCloth,

	ConstructionBuilding,
	FarmBuilding,
	CampBuilding,
	SawmillBuilding,
	StoneMineBuilding,
	IronMineBuilding,
	WorkshopBuilding,
	CastleBuilding,

	SpawnerRedBuilding,
	SpawnerBlueBuilding,

	Bag,
	Cursor,
}

const TEXTURE_COUNT: usize = 18;

pub struct TextureState {
	wrappers: [Texture; TEXTURE_COUNT],
}

fn get_image_path(s: &str) -> String {
	use toalib::misc::res_dir;

	let mut dir = res_dir();
	dir.push("image");
	let path_string = dir.to_str().unwrap();
	format!("{}/{}", path_string, s)
}

impl TextureState {
	pub fn new() -> TextureState {

		unsafe {
			let mut wrappers: [Texture; TEXTURE_COUNT] = mem::uninitialized();

			let nope_texture = Texture::from_file(&get_image_path("nope.png")).unwrap();

			macro_rules! load {
				($a: expr, $b: expr) => {{
					let texture = Texture::from_file(&get_image_path($b)).unwrap_or(nope_texture.clone());
					ptr::write(&mut wrappers[$a as usize], texture);
				}}
			}

			load!(TextureId::GrassTerrain, "terrain/grass.png");
			load!(TextureId::ForestTerrain, "terrain/forest.png");
			load!(TextureId::StoneTerrain, "terrain/stone.png");
			load!(TextureId::IronTerrain, "terrain/iron.png");

			load!(TextureId::Unit, "unit.png");
			load!(TextureId::UnitCloth, "unit_cloth.png");

			load!(TextureId::ConstructionBuilding, "building/construction.png");
			load!(TextureId::FarmBuilding, "building/farm.png");
			load!(TextureId::CampBuilding, "building/camp.png");
			load!(TextureId::SawmillBuilding, "building/sawmill.png");
			load!(TextureId::StoneMineBuilding, "building/stonemine.png");
			load!(TextureId::IronMineBuilding, "building/ironmine.png");
			load!(TextureId::WorkshopBuilding, "building/workshop.png");
			load!(TextureId::CastleBuilding, "building/castle.png");

			load!(TextureId::SpawnerRedBuilding, "building/spawner/red.png");
			load!(TextureId::SpawnerBlueBuilding, "building/spawner/blue.png");

			load!(TextureId::Bag, "bag.png");
			load!(TextureId::Cursor, "cursor.png");

			TextureState { wrappers }
		}
	}

	pub fn get_texture(&self, id: TextureId) -> &Texture {
		&self.wrappers[id as usize]
	}
}
