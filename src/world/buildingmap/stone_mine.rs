use std::any::Any;

use crate::vec::Pos;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::damage::Damage;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Wood, ItemClass::Wood, ItemClass::Wood, ItemClass::Wood],
		stamina_cost: 0,
		build: || Building::StoneMine(StoneMine { health: 100 }),
		required_terrain: Some(Terrain::STONE),
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct StoneMineClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct StoneMine {
	health: u32,
}

impl BuildingClassTrait for StoneMineClass {
	type Instance = StoneMine;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"StoneMine"
	}
}

impl BuildingTrait for StoneMine {
	type Class = StoneMineClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::StoneMine }
	fn is_burnable(&self, _w: &World, _p: Pos) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Pos) -> bool { true }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Pos) {
		let u = w.unitmap.get_mut(p).unwrap();
		u.inventory.push(ItemClass::Stone.build());
	}
	fn get_info_string(&self) -> String {
		format!("StoneMine( health: {})", self.health)
	}

}
