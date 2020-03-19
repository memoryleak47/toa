use toalib::item::{Item, ItemClass};
use toalib::vec::Vec2f;

use crate::graphics::{RawTextureId, TextureId, HasTexture, GameObject};

impl HasTexture for Item {
	fn get_texture_id(&self) -> TextureId {
		self.get_class().get_texture_id()
	}
}

impl HasTexture for ItemClass {
	fn get_texture_id(&self) -> TextureId {
		match *self {
			ItemClass::Food => RawTextureId::FoodItem,
			ItemClass::Wood => RawTextureId::WoodItem,
			ItemClass::WoodSword => RawTextureId::WoodSwordItem,
			ItemClass::Stone => RawTextureId::StoneItem,
			ItemClass::Iron => RawTextureId::IronItem,
			ItemClass::IronSword => RawTextureId::IronSwordItem,
			ItemClass::WoodBow => RawTextureId::WoodBowItem,
			ItemClass::SettlementKit => RawTextureId::IronItem,
			ItemClass::LongSword => RawTextureId::LongSwordItem,
			ItemClass::Lance => RawTextureId::LanceItem,
		}.into()
	}
}

impl GameObject for Item {
	fn get_relative_pos(&self) -> Vec2f { (0., 0.5).into() }
	fn get_size(&self) -> Vec2f { (0.25, 0.5).into() }
}
