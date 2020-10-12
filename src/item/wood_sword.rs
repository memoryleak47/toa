use crate::*;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct WoodSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct WoodSword;

impl ItemClassTrait for WoodSwordClass {
	type Instance = WoodSword;

	fn get_name() -> &'static str { "WoodSword" }
	fn get_weight() -> u32 { 8 }
	fn build() -> Item {
		Item::WoodSword(WoodSword)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for WoodSword {
	type Class = WoodSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::WoodSword
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn get_damage(&self) -> Damage { Damage(6) }
}
