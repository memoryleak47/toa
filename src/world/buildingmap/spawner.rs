use sfml::graphics::Color;

use item::{Inventory, ItemKind};
use super::{BuildingClass, Building};
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref TEAM_SPAWNER_COLOR: [Color; 2] = [Color::rgb(100, 0, 0), Color::rgb(0, 100, 0)];
}
pub static SPAWNER_CLASS: SpawnerClass = SpawnerClass;

pub struct SpawnerClass;

pub struct Spawner {
	player: u32,
	health: u32,
	required_food: u32,
}

impl BuildingClass for SpawnerClass {
	fn get_required_terrain(&self) -> Option<Terrain> { None }
	fn get_build_cost(&self) -> Inventory {
		panic!("you should call get_build_cost() on Spawner!")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<Building> {
		panic!("you should never call build() on Spawner!")
	}
}

impl Building for Spawner {
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { &SPAWNER_CLASS }
	fn is_burnable(&self, unit: &Unit) -> bool { false }
	fn is_workable(&self, unit: &Unit) -> bool {
		unit.inventory.contains_all(&[ItemKind::Food][..])
	}
	fn get_color(&self) -> &'static Color {
		&TEAM_SPAWNER_COLOR[self.player as usize]
	}
}

impl Spawner {
	pub fn new(player: u32) -> Spawner {
		Spawner { player, health: 100, required_food: 10 } // TODO un-hardcode
	}

	pub fn new_boxed(player: u32) -> Box<Building> {
		Box::new(Spawner::new(player))
	}
}
