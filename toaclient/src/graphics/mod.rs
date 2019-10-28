pub mod terrain;
pub mod building;
pub mod item;

use sfml::graphics::Texture;

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		pub enum TextureId {
			$($x),*
		}

		impl TextureState {
			pub fn new() -> TextureState {
				let nope_texture = load_texture("nope.png").unwrap();
				let mut textures = Vec::new();
				$( textures.push(load_texture($y).unwrap_or(nope_texture.clone())); );*
				TextureState { textures }
			}
		}
	};
}

pub struct TextureState {
	textures: Vec<Texture>,
}

fn load_texture(s: &str) -> Option<Texture> {
	use crate::misc::resource;

	let p = resource(&format!("image/{}", s));
	Texture::from_file(&p)
}

impl TextureState {
	pub fn get_texture(&mut self, tid: TextureId) -> &Texture {
		&self.textures[tid as usize]
	}
}

setup!(
	GrassTerrain: "terrain/grass.png",
	ForestTerrain: "terrain/forest.png",
	StoneTerrain: "terrain/stone.png",
	IronTerrain: "terrain/iron.png",
	MountainTerrain: "terrain/mountain.png",
	MarshTerrain: "terrain/marsh.png",

	Unit: "unit.png",

	FarmBuilding: "building/farm.png",
	CampBuilding: "building/camp.png",
	SawmillBuilding: "building/sawmill.png",
	StoneMineBuilding: "building/stonemine.png",
	IronMineBuilding: "building/ironmine.png",
	WorkshopBuilding: "building/workshop.png",
	CastleBuilding: "building/castle.png",
	WoodWallBuilding: "building/woodwall.png",
	StoneWallBuilding: "building/stonewall.png",
	StreetBuilding: "building/street.png",

	FoodItem: "item/food.png",
	WoodItem: "item/wood.png",
	WoodSwordItem: "item/woodsword.png",
	StoneItem: "item/stone.png",
	IronItem: "item/iron.png",
	IronSwordItem: "item/ironsword.png",
	WoodBowItem: "item/woodbow.png",
	LongSwordItem: "item/longsword.png",
	LanceItem: "item/lance.png",

	Bag: "bag.png",
	Cursor: "cursor.png",
	CombatCursor: "combat_cursor.png",

	SpawnerBuilding: "building/spawner_template.png",
	UnitCloth: "unit_cloth_template.png"
);
