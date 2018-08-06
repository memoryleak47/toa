use std::ptr;
use std::mem;

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
	SpawnerRedBuilding,
	SpawnerBlueBuilding,

	Bag,
}

const TEXTURE_COUNT: usize = 11;

pub struct TextureState {
	wrappers: [Texture; TEXTURE_COUNT],
}

fn get_res_path(s: &str) -> String {
	let dir = ::misc::res_dir();
	let path_string = dir.to_str().unwrap();
	format!("{}/{}", path_string, s)
}

impl TextureState {
	pub fn new() -> TextureState {

		unsafe {
			let mut wrappers: [Texture; TEXTURE_COUNT] = mem::uninitialized();

			let nope_texture = Texture::from_file(&get_res_path("nope.png")).unwrap();

			macro_rules! load {
				($a: expr, $b: expr) => {{
					let texture = Texture::from_file(&get_res_path($b)).unwrap_or(nope_texture.clone());
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

			load!(TextureId::SpawnerRedBuilding, "building/spawner/red.png");
			load!(TextureId::SpawnerBlueBuilding, "building/spawner/blue.png");

			load!(TextureId::Bag, "bag.png");

			TextureState { wrappers }
		}
	}

	pub fn get_texture(&self, id: TextureId) -> &Texture {
		&self.wrappers[id as usize]
	}
}
