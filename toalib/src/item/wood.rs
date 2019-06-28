use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct WoodClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Wood;

impl ItemClassTrait for WoodClass {
	type Instance = Wood;

	fn get_name() -> &'static str { "Wood" }
	fn get_weight() -> u32 { 4 }
	fn build() -> Item {
		Item::Wood(Wood)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { None }
}

impl ItemTrait for Wood {
	type Class = WoodClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Wood
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn aim(&self) -> Aim {
		new_meelee_aim(Damage(5))
	}
}
