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
		build: || Building::Farm(Farm { health: 100 }),
		required_terrain: Some(Terrain::GRASS),
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct FarmClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Farm {
	health: u32,
}

impl BuildingClassTrait for FarmClass {
	type Instance = Farm;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"Farm"
	}
}

impl BuildingTrait for Farm {
	type Class = FarmClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Farm }
	fn is_burnable(&self, _w: &World, _p: Pos) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Pos) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Pos) {
		panic!("you cannot work at farm!")
	}
	fn get_info_string(&self) -> String {
		format!("Farm( health: {})", self.health)
	}

}
