use crate::vec::Vec2u;
use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::World;
use crate::world::aim::{Aim, MeeleeAim};
use crate::world::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FoodClass;

#[derive(Clone)]
pub struct Food;

impl ItemClassTrait for FoodClass {
	type Instance = Food;
	
	fn get_name() -> &'static str { "Food" }
	fn get_weight() -> u32 { 10 }
	fn build() -> Item {
		Item::Food(Food)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { None }
}

impl ItemTrait for Food {
	type Class = FoodClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::Food
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new(Damage(1)))
	}
	fn is_execable(&self, _p: Vec2u, _w: &World) -> bool { true }
	fn exec(&self, p: Vec2u, w: &mut World) {
		w.get_unit_mut(p).unwrap()
			.food += 20;
	}
}