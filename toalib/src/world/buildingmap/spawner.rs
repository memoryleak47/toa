use std::any::Any;

use crate::vec::Vec2u;
use crate::world::buildingmap::{BuildingClass, Building, BuildingClassTrait, BuildingTrait, BuildProperty};
use crate::world::World;
use crate::damage::Damage;
use crate::team::PlayerID;

// see tick_spawners()!

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SpawnerClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Spawner {
	player: PlayerID,
	health: u32,
}

impl BuildingClassTrait for SpawnerClass {
	type Instance = Spawner;

	fn get_build_property() -> Option<&'static BuildProperty> { None }
	fn get_height() -> u32 { 0 }
	fn get_name() -> &'static str {
		"Spawner"
	}
}

impl BuildingTrait for Spawner {
	type Class = SpawnerClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Spawner }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _w: &mut World, _p: Vec2u) { }
	fn get_info_string(&self) -> String {
		format!("Spawner( health: {}, player: {})", self.health, self.player)
	}
	fn is_blocking_against(&self, pid: PlayerID) -> bool { pid != self.player }


}

impl Spawner {
	pub fn get_player_id(&self) -> PlayerID {
		self.player
	}
}

pub fn new_spawner(player: PlayerID) -> Building {
	Building::Spawner(Spawner { player, health: 300}) // TODO un-hardcode
}
