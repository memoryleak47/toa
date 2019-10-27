use crate::vec::Pos;
use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::World;
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct FoodClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Food;

impl ItemClassTrait for FoodClass {
	type Instance = Food;
	
	fn get_name() -> &'static str { "Food" }
	fn get_weight() -> u32 { 2 }
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
	fn aim(&self) -> Aim {
		new_meelee_aim(Damage(1))
	}
	fn is_execable(&self, _p: Pos, _w: &World) -> bool { true }
	fn exec(&self, p: Pos, w: &mut World) {
		w.get_unit_mut(p).unwrap()
			.food += 20;
	}
}
