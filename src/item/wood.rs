use item::{Item, ItemClass};

pub struct WoodClass;

pub struct Wood {
	health: u32,
}

impl ItemClass for WoodClass {
	fn get_ref(&self) -> &'static ItemClass {
		&WoodClass
	}
	fn get_mass(&self) -> u32 {
		10
	}
	fn build(&self) -> Box<Item> {
		Box::new(Wood { health: 10 })
	}
}

impl Item for Wood {
	fn get_class(&self) -> &'static ItemClass {
		WoodClass.get_ref()
	}
	fn damage(&mut self) {
		self.health = self.health.saturating_sub(1);
	}
	fn is_dead(&self) -> bool {
		self.health == 0
	}
}
