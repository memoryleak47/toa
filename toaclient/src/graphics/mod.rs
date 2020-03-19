mod hue;
pub mod terrain;
pub mod building;
pub mod item;
pub mod bag;
pub mod unit;

use std::collections::HashMap;

use sfml::graphics::{Texture, Color};

use toalib::vec::Vec2f;
use toalib::team::{PlayerID, COLORS};

// TextureId % (COLORS.len()+1) = hue (0 means no hue, i=1.. corresponds to PlayerID i-1)
// TextureId / (COLORS.len()+1) = raw_img

// the non-hued graphics are all loaded on startup in (TextureState::new)
// the hued graphics are loaded lazily using lazy_load

macro_rules! setup {
	($($x:ident : $y:expr),*) => {

		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		pub enum RawTextureId {
			$($x),*
		}

		impl TextureState {
			pub fn new() -> TextureState {
				let nope_texture = load_texture("nope.png").unwrap();
				let mut wrappers = HashMap::new();
				let mut i = 0;
				$( {
					i += 1;
					wrappers.insert(TextureId((i-1) * (COLORS.len() + 1)), load_texture($y).unwrap_or(nope_texture.clone()));
				}; )*
				TextureState { wrappers }
			}
		}
	};
}

pub trait GameObject {
	fn get_texture_id(&self) -> TextureId;
	fn get_hue(&self) -> Option<Color> { None }
	fn get_relative_pos(&self) -> Vec2f;	// position (has to be added to its position); in tile-coordinates
	fn get_size(&self) -> Vec2f;			// in tile-coordinates
}

#[derive(Debug)]
pub struct HuedTextureId {
	pub raw: RawTextureId,
	pub player_id: PlayerID,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct TextureId(pub usize);

pub struct TextureState {
	wrappers: HashMap<TextureId, Texture>,
}

fn load_texture(s: &str) -> Option<Texture> {
	use crate::misc::resource;

	let p = resource(&format!("image/{}", s));
	Texture::from_file(&p)
}

impl From<HuedTextureId> for TextureId {
	fn from(hued: HuedTextureId) -> TextureId {
		TextureId(hued.raw as usize * (COLORS.len() + 1) + hued.player_id.0 + 1)
	}
}

impl From<RawTextureId> for TextureId {
	fn from(raw: RawTextureId) -> TextureId {
		TextureId(raw as usize * (COLORS.len() + 1))
	}
}

impl TextureState {
	fn lazy_load(&mut self, tid: TextureId) {
		if self.wrappers.get(&tid).is_some() { return; }

		let tmp = (tid.0 / (COLORS.len()+1)) * (COLORS.len()+1);
		let raw = self.wrappers.get(&TextureId(tmp)).unwrap();
		let color_id = tid.0 % (COLORS.len()+1); 
		let tex = hue::hue(raw, COLORS[color_id-1]);
		self.wrappers.insert(tid, tex);
	}

	pub fn get_texture<T: Into<TextureId>>(&mut self, id: T) -> &Texture {
		let tid = id.into();
		self.lazy_load(tid);
		self.wrappers.get(&tid).unwrap()
	}
}

setup!(
	// non-hued:

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

	// hued:

	SpawnerBuilding: "building/spawner_template.png",
	UnitCloth: "unit_cloth_template.png"
);
