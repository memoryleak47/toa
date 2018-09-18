use crate::item::{Item, ItemClass, ItemBox};
use crate::item::wood::WoodClass;
use crate::world::aim::{Aim, MeeleeAim};
use crate::world::damage::Damage;

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
	fn build(&self) -> ItemBox {
		ItemBox(Box::new(Club { health: 100 }))
	}
	fn get_recipe(&self) -> Option<&'static [&'static dyn ItemClass]> { Some(&RECIPE[..]) }
}

impl Item for Club {
	fn get_class(&self) -> &'static dyn ItemClass {
		ClubClass.get_ref()
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn clone_box(&self) -> ItemBox {
		ItemBox(Box::new(self.clone()))
	}
	fn aim(&self) -> Box<dyn Aim> {
		Box::new(MeeleeAim::new(Damage(10)))
	}
}
