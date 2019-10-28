use toalib::item::ItemClass;

use crate::graphics::{RawTextureId, TextureId};

pub fn get_texture_id(class: ItemClass) -> TextureId {
	match class {
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
