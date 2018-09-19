use std::any::Any;

use crate::vec::Vec2u;
use crate::item::{ItemClass};
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct ConstructionClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Construction {
	health: u32,
	invested_stamina: u32,
	build_class: BuildingClass,
}

impl BuildingClassTrait for ConstructionClass {
	type Instance = Construction;

	fn get_required_terrain() -> Option<Terrain> {
		panic!("get_required_terrain() should not be called on a Construction")
	}
	fn get_build_item_cost() -> &'static [ItemClass] {
		panic!("get_build_item_cost() should not be called on a Construction")
	}
	fn get_build_stamina_cost() -> u32 {
		panic!("get_build_stamina_cost() should not be called on a Construction")
	}
	fn get_height() -> u32 { 0 }

	fn build() -> Building {
		panic!("build() should not be called on a Construction")
	}
	fn get_name() -> &'static str {
		"Construction"
	}
}

impl BuildingTrait for Construction {
	type Class = ConstructionClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Construction }
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

pub fn new_construction(class: BuildingClass) -> Building {
		Building::Construction(
			Construction {
				health: 100, // TODO un-hardcode
				invested_stamina: 0,
				build_class: class,
			}
		)
}

