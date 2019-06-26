use std::any::Any;

use crate::vec::Vec2u;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::damage::Damage;

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

	fn get_build_property() -> Option<&'static BuildProperty> { None }
	fn get_height() -> u32 { 0 }
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
		let property = self.build_class.get_build_property().unwrap();
		self.invested_stamina += 10; // TODO make correct
		if self.invested_stamina >= property.stamina_cost {
			let b = (property.build)();
			world.set_building(p, Some(b));
		}
	}
	fn get_info_string(&self) -> String {
		format!("Construction( health: {}, {}/{}, building: {})", self.health, self.invested_stamina, self.build_class.get_build_property().unwrap().stamina_cost, self.build_class.get_name())
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

