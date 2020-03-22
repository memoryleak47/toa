use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct StoneClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Stone;

impl ItemClassTrait for StoneClass {
	type Instance = Stone;

	fn get_name() -> &'static str { "Stone" }
	fn get_weight() -> u32 { 10 }
	fn build() -> Item {
		Item::Stone(Stone)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { None }
	fn stateless() -> bool { true }
}

impl ItemTrait for Stone {
	type Class = StoneClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Stone
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn aim(&self) -> Aim {
		new_meelee_aim(Damage(7))
	}
}
