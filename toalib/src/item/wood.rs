use crate::item::{Item, ItemClass, ItemBox};
use crate::world::aim::{Aim, MeeleeAim};
use crate::world::damage::Damage;

pub struct WoodClass;

#[derive(Clone)]
pub struct Wood;

impl ItemClass for WoodClass {
	fn get_name(&self) -> &'static str { "Wood" }
	fn get_ref(&self) -> &'static dyn ItemClass {
		&WoodClass
	}
	fn get_weight(&self) -> u32 {
		10
	}
	fn build(&self) -> ItemBox {
		ItemBox(Box::new(Wood))
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { None }
}

impl Item for Wood {
	fn get_class(&self) -> &'static dyn ItemClass {
		WoodClass.get_ref()
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn clone_box(&self) -> ItemBox {
		ItemBox(Box::new(self.clone()))
	}
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new(Damage(5)))
	}
}
