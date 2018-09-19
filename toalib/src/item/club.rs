use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::aim::{Aim, MeeleeAim};
use crate::world::damage::Damage;

lazy_static! {
	static ref RECIPE: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ClubClass;

#[derive(Clone)]
pub struct Club {
	health: u32,
}

impl ItemClassTrait for ClubClass {
	type Instance = Club;

	fn get_name() -> &'static str { "Club" }
	fn get_weight() -> u32 { 100 }
	fn build() -> Item {
		Item::Club(Club { health: 100 })
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
}

impl ItemTrait for Club {
	type Class = ClubClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Club
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new(Damage(10)))
	}
}
