use std::any::Any;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::damage::Damage;
use crate::team::PlayerID;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Stone],
		stamina_cost: 40,
		build: || Building::StoneWall(StoneWall { health: 100 }),
		required_terrain: None,
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct StoneWallClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct StoneWall {
	health: u32,
}

impl BuildingClassTrait for StoneWallClass {
	type Instance = StoneWall;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"StoneWall"
	}
}

impl BuildingTrait for StoneWall {
	type Class = StoneWallClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::StoneWall }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _w: &mut World, _p: Vec2u) {
		panic!("can't work on stone wall");
	}
	fn get_info_string(&self) -> String {
		format!("StoneWall( health: {})", self.health)
	}
	fn is_blocking_against(&self, _pid: PlayerID) -> bool { true }
}
