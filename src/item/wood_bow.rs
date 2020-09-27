use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::vec::{Vec2i, Vec2f};
use crate::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct WoodBowClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct WoodBow {
	health: u32,
}

impl ItemClassTrait for WoodBowClass {
	type Instance = WoodBow;

	fn get_name() -> &'static str { "WoodBow" }
	fn get_weight() -> u32 { 15 }
	fn build() -> Item {
		Item::WoodBow(WoodBow { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for WoodBow {
	type Class = WoodBowClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::WoodBow
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn get_damage(&self) -> Damage { Damage(3) }
	fn aim(&self, v: Vec2f) -> Vec<Vec2i> {
		// TODO
		//new_ranged_aim(3, Damage(3))
		vec![]
	}
}
