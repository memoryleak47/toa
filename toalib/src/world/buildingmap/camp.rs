use std::any::Any;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::damage::Damage;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Wood, ItemClass::Wood],
		stamina_cost: 20,
		build: || Building::Camp(Camp { health: 100 }),
		required_terrain: None,
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct CampClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Camp {
	health: u32,
}

impl BuildingClassTrait for CampClass {
	type Instance = Camp;

	fn get_height() -> u32 { 0 }
	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"Camp"
	}
}

impl BuildingTrait for Camp {
	type Class = CampClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Camp }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _w: &mut World, _p: Vec2u) {
		panic!("can't work on camp");
	}

}
