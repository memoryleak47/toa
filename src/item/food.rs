use sfml::system::Vector2u;

use item::{Item, ItemClass, ItemBox};
use world::World;
use world::aim::{Aim, MeeleeAim};
use world::damage::Damage;

pub struct FoodClass;

#[derive(Clone)]
pub struct Food;

impl ItemClass for FoodClass {
	fn get_name(&self) -> &'static str { "Food" }
	fn get_ref(&self) -> &'static dyn ItemClass {
		&FoodClass
	}
	fn get_weight(&self) -> u32 {
		10
	}
	fn build(&self) -> ItemBox {
		ItemBox(Box::new(Food))
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { None }
}

impl Item for Food {
	fn get_class(&self) -> &'static dyn ItemClass {
		FoodClass.get_ref()
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn clone_box(&self) -> ItemBox {
		ItemBox(Box::new(self.clone()))
	}
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new(Damage(1)))
	}
	fn is_execable(&self, _p: Vector2u, _w: &World) -> bool { true }
	fn exec(&self, p: Vector2u, w: &mut World) {
		w.get_unit_mut(p).unwrap()
			.food += 20;
	}
}
