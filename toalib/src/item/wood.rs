use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::aim::{Aim, new_meelee_aim};
use crate::world::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct WoodClass;

#[derive(Clone)]
pub struct Wood;

impl ItemClassTrait for WoodClass {
	type Instance = Wood;

	fn get_name() -> &'static str { "Wood" }
	fn get_weight() -> u32 { 10 }
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
