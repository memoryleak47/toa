use toalib::item::ItemClass;

use crate::graphics::TextureId;

pub fn get_texture_id(class: ItemClass) -> TextureId {
	match class {
		ItemClass::Food => TextureId::FoodItem,
		ItemClass::Wood => TextureId::WoodItem,
		ItemClass::WoodSword => TextureId::WoodSwordItem,
		ItemClass::Stone => TextureId::StoneItem,
		ItemClass::Iron => TextureId::IronItem,
		ItemClass::IronSword => TextureId::IronSwordItem,
		ItemClass::WoodBow => TextureId::WoodBowItem,
        ItemClass::SettlementKit => TextureId::IronItem,
        ItemClass::LongSword => TextureId::LongSwordItem,
        ItemClass::Lance => TextureId::LanceItem,
	}
}
