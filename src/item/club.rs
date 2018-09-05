use std::cmp::max;

use item::{Item, ItemClass};
use item::wood::WoodClass;

lazy_static! {
	static ref RECIPE: [&'static dyn ItemClass; 2] = [WoodClass.get_ref(), WoodClass.get_ref()];
}

pub struct ClubClass;

#[derive(Clone)]
pub struct Club {
	health: u32,
}

impl ItemClass for ClubClass {
	fn get_name(&self) -> &'static str { "Club" }
	fn get_ref(&self) -> &'static dyn ItemClass {
		&ClubClass
	}
	fn get_weight(&self) -> u32 {
		100
	}
	fn build(&self) -> Box<dyn Item> {
		Box::new(Club { health: 100 })
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { Some(&RECIPE[..]) }
}

impl Item for Club {
	fn get_class(&self) -> &'static dyn ItemClass {
		ClubClass.get_ref()
	}
	fn damage(&mut self) {
		self.health = max(self.health, 0);
	}
	fn is_dead(&self) -> bool {
		self.health == 0
	}
}
