use std::any::Any;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::damage::Damage;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Wood, ItemClass::Wood, ItemClass::Wood, ItemClass::Wood],
		stamina_cost: 80,
		build: || Building::Sawmill(Sawmill { health: 100 }),
		required_terrain: Some(Terrain::FOREST),
	};

}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SawmillClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sawmill {
	health: u32,
}

impl BuildingClassTrait for SawmillClass {
	type Instance = Sawmill;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_height() -> u32 { 0 }
	fn get_name() -> &'static str {
		"Sawmill"
	}
}

impl BuildingTrait for Sawmill {
	type Class = SawmillClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Sawmill }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Vec2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.push(ItemClass::Wood.build());
	}
	fn get_info_string(&self) -> String {
		format!("Sawmill( health: {})", self.health)
	}

}
