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
		build: || Building::IronMine(IronMine { health: 100 }),
		required_terrain: Some(Terrain::IRON),
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct IronMineClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct IronMine {
	health: u32,
}

impl BuildingClassTrait for IronMineClass {
	type Instance = IronMine;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"IronMine"
	}
}

impl BuildingTrait for IronMine {
	type Class = IronMineClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::IronMine }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Vec2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.push(ItemClass::Iron.build());
	}
	fn get_info_string(&self) -> String {
		format!("IronMine( health: {})", self.health)
	}

}
