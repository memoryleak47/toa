use std::any::Any;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;

lazy_static! {
	static ref BUILD_ITEM_COST: [ItemClass; 2] = [ItemClass::Wood, ItemClass::Wood];
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

	fn get_required_terrain() -> Option<Terrain> { Some(Terrain::FOREST) }
	fn get_build_item_cost() -> &'static [ItemClass] {
		&BUILD_ITEM_COST[..]
	}
	fn get_build_stamina_cost() -> u32 { 20 }
	fn get_height() -> u32 { 0 }
	fn build() -> Building {
		Building::Sawmill(Sawmill { health: 100 })
	}
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
}