use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::aim::{Aim, new_lance_aim};
use crate::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Iron];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LanceClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Lance {
	health: u32,
}

impl ItemClassTrait for LanceClass {
	type Instance = Lance;

	fn get_name() -> &'static str { "Lance" }
	fn get_weight() -> u32 { 40 }
	fn build() -> Item {
		Item::Lance(Lance { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
}

impl ItemTrait for Lance {
	type Class = LanceClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Lance
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn aim(&self) -> Aim {
		new_lance_aim(Damage(12))
	}
}

