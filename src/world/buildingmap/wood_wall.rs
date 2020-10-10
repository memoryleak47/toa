use std::any::Any;

use crate::vec::Pos;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::damage::Damage;
use crate::team::PlayerID;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Wood, ItemClass::Wood],
		stamina_cost: 0,
		build: || Building::WoodWall(WoodWall { health: 100 }),
		required_terrain: None,
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct WoodWallClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct WoodWall {
	health: u32,
}

impl BuildingClassTrait for WoodWallClass {
	type Instance = WoodWall;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"WoodWall"
	}
}

impl BuildingTrait for WoodWall {
	type Class = WoodWallClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::WoodWall }
	fn is_burnable(&self, _w: &World, _p: Pos) -> bool { false }
	fn is_workable(&self, _w: &World, _p: Pos) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _w: &mut World, _p: Pos) {
		panic!("can't work on wood wall");
	}
	fn get_info_string(&self) -> String {
		format!("WoodWall( health: {})", self.health)
	}
	fn is_blocking_against(&self, _pid: PlayerID) -> bool { true }
}

