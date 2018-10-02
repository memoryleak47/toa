use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::aim::{Aim, new_meelee_aim};
use crate::world::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct WoodSwordClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct WoodSword {
	health: u32,
}

impl ItemClassTrait for WoodSwordClass {
	type Instance = WoodSword;

	fn get_name() -> &'static str { "WoodSword" }
	fn get_weight() -> u32 { 100 }
	fn build() -> Item {
		Item::WoodSword(WoodSword { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
}

impl ItemTrait for WoodSword {
	type Class = WoodSwordClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::WoodSword
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn aim(&self) -> Aim {
		new_meelee_aim(Damage(10))
	}
}