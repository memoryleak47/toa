use std::any::Any;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;

pub struct ConstructionClass;

#[derive(Clone)]
pub struct Construction {
	health: u32,
	invested_stamina: u32,
	build_class: &'static dyn BuildingClass,
}

impl BuildingClass for ConstructionClass {
	fn get_ref(&self) -> &'static dyn BuildingClass { &ConstructionClass }
	fn get_required_terrain(&self) -> Option<Terrain> {
		panic!("get_required_terrain() should not be called on a Construction")
	}
	fn get_build_item_cost(&self) -> &'static [&'static dyn ItemClass] {
		panic!("get_build_item_cost() should not be called on a Construction")
	}
	fn get_build_stamina_cost(&self) -> u32 {
		panic!("get_build_stamina_cost() should not be called on a Construction")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<dyn Building> {
		panic!("build() should not be called on a Construction")
	}
	fn get_name(&self) -> &'static str {
		"Construction"
	}
}

impl Building for Construction {
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> &'static dyn BuildingClass { ConstructionClass.get_ref() }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, world: &mut World, p: Vec2u) {
		self.invested_stamina += 10; // TODO make correct
		if self.invested_stamina >= self.build_class.get_build_stamina_cost() {
			let b = self.build_class.build();
			world.set_building(p, Some(b));
		}
	}
}

impl Construction {
	pub fn new(class: &'static dyn BuildingClass) -> Construction {
		Construction {
			health: 100, // TODO un-hardcode
			invested_stamina: 0,
			build_class: class,
		}
	}
}
