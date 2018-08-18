use item::{Item, ItemClass};

pub struct FoodClass;

pub struct Food {
	alive: bool,
}

impl ItemClass for FoodClass {
	fn get_name(&self) -> &'static str { "Food" }
	fn get_ref(&self) -> &'static ItemClass {
		&FoodClass
	}
	fn get_mass(&self) -> u32 {
		10
	}
	fn build(&self) -> Box<Item> {
		Box::new(Food { alive: true })
	}
}

impl Item for Food {
	fn get_class(&self) -> &'static ItemClass {
		FoodClass.get_ref()
	}
	fn damage(&mut self) {
		self.alive = false;
	}
	fn is_dead(&self) -> bool {
		!self.alive
	}
}
