use toalib::item::ItemClass;

use crate::graphics::TextureId;

pub fn get_texture_id(class: ItemClass) -> TextureId {
	match class {
		ItemClass::Food => TextureId::FoodItem,
		ItemClass::Wood => TextureId::WoodItem,
		ItemClass::WoodSword => TextureId::WoodSwordItem,
		ItemClass::Stone => TextureId::StoneItem,
		ItemClass::Iron => TextureId::IronItem,
	}
}