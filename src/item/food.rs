use item::{Item, ItemClass, ItemBox};
use world::aim::{Aim, MeeleeAim};

pub struct FoodClass;

#[derive(Clone)]
pub struct Food {
	alive: bool,
}

impl ItemClass for FoodClass {
	fn get_name(&self) -> &'static str { "Food" }
	fn get_ref(&self) -> &'static dyn ItemClass {
		&FoodClass
	}
	fn get_weight(&self) -> u32 {
		10
	}
	fn build(&self) -> ItemBox {
		ItemBox(Box::new(Food { alive: true }))
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { None }
}

impl Item for Food {
	fn get_class(&self) -> &'static dyn ItemClass {
		FoodClass.get_ref()
	}
	fn damage(&mut self) {
		self.alive = false;
	}
	fn is_dead(&self) -> bool {
		!self.alive
	}
	fn clone_box(&self) -> ItemBox {
		ItemBox(Box::new(self.clone()))
	}
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new())
	}
}
