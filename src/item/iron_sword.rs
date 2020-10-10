use crate::*;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Iron, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct IronSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct IronSword;

impl ItemClassTrait for IronSwordClass {
	type Instance = IronSword;

	fn get_name() -> &'static str { "IronSword" }
	fn get_weight() -> u32 { 15 }
	fn build() -> Item {
		Item::IronSword(IronSword)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for IronSword {
	type Class = IronSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::IronSword
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn get_damage(&self) -> Damage { Damage(20) }
}
