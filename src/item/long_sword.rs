use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 3] = [ItemClass::Iron, ItemClass::Iron, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LongSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct LongSword {
	health: u32,
}

impl ItemClassTrait for LongSwordClass {
	type Instance = LongSword;

	fn get_name() -> &'static str { "LongSword" }
	fn get_weight() -> u32 { 40 }
	fn build() -> Item {
		Item::LongSword(LongSword { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for LongSword {
	type Class = LongSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::LongSword
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn get_damage(&self) -> Damage { Damage(15) }
}
