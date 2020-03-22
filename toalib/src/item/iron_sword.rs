use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Iron, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct IronSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct IronSword {
	health: u32,
}

impl ItemClassTrait for IronSwordClass {
	type Instance = IronSword;

	fn get_name() -> &'static str { "IronSword" }
	fn get_weight() -> u32 { 40 }
	fn build() -> Item {
		Item::IronSword(IronSword { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for IronSword {
	type Class = IronSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::IronSword
	}
	fn inflict_damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn get_damage(&self) -> Damage { Damage(20) }
}
