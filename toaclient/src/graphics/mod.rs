pub mod terrain;
pub mod building;

use sfml::graphics::Texture;

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone)]
		#[repr(usize)]
		pub enum TextureId {
			$($x),*
		}

		pub struct TextureState {
			wrappers: Vec<Texture>,
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
				let nope_texture = Texture::from_file(&get_image_path("nope.png")).unwrap();
				let wrappers = vec![$( Texture::from_file(&get_image_path($y)).unwrap_or(nope_texture.clone())),*];
				TextureState { wrappers }
			}

			pub fn get_texture(&self, id: TextureId) -> &Texture {
				&self.wrappers[id as usize]
			}
		}
	};
}

setup!(
	GrassTerrain: "terrain/grass.png",
	ForestTerrain: "terrain/forest.png",
	StoneTerrain: "terrain/stone.png",
	IronTerrain: "terrain/iron.png",

	Unit: "unit.png",
	UnitCloth: "unit_cloth.png",

	ConstructionBuilding: "building/construction.png",
	FarmBuilding: "building/farm.png",
	CampBuilding: "building/camp.png",
	SawmillBuilding: "building/sawmill.png",
	StoneMineBuilding: "building/stonemine.png",
	IronMineBuilding: "building/ironmine.png",
	WorkshopBuilding: "building/workshop.png",
	CastleBuilding: "building/castle.png",

	SpawnerRedBuilding: "building/spawner/red.png",
	SpawnerBlueBuilding: "building/spawner/blue.png",

	Bag: "bag.png",
	Cursor: "cursor.png"
);
