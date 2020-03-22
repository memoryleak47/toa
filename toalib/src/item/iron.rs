use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct IronClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Iron;

impl ItemClassTrait for IronClass {
	type Instance = Iron;

	fn get_name() -> &'static str { "Iron" }
	fn get_weight() -> u32 { 10 }
	fn build() -> Item {
		Item::Iron(Iron)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { None }
	fn stateless() -> bool { true }
}

impl ItemTrait for Iron {
	type Class = IronClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Iron
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn aim(&self) -> Aim {
		new_meelee_aim(Damage(8))
	}
}
