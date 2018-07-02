use item::{Item, ItemClass};

pub struct FoodClass;

pub struct Food {
	health: u32,
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
		Box::new(Food { health: 5 })
	}
}

impl Item for Food {
	fn get_class(&self) -> &'static ItemClass {
		FoodClass.get_ref()
	}
	fn damage(&mut self) {
		self.health = self.health.saturating_sub(1);
	}
	fn is_dead(&self) -> bool {
		self.health == 0
	}
}
