use item::{Item, ItemClass};

pub struct WoodClass;

#[derive(Clone)]
pub struct Wood {
	alive: bool,
}

impl ItemClass for WoodClass {
	fn get_name(&self) -> &'static str { "Wood" }
	fn get_ref(&self) -> &'static dyn ItemClass {
		&WoodClass
	}
	fn get_mass(&self) -> u32 {
		10
	}
	fn build(&self) -> Box<dyn Item> {
		Box::new(Wood { alive: true })
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { None }
}

impl Item for Wood {
	fn get_class(&self) -> &'static dyn ItemClass {
		WoodClass.get_ref()
	}
	fn damage(&mut self) {
		self.alive = false;
	}
	fn is_dead(&self) -> bool {
		!self.alive
	}
}
