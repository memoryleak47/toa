use std::any::Any;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item::ItemKind;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref TEAM_SPAWNER_COLOR: [Color; 2] = [Color::rgb(100, 0, 0), Color::rgb(0, 100, 0)];
	static ref WORK_FN: fn(&mut World, Vector2u) = |w, p| {
		// TODO reduce food, create new dude
	};
}

pub struct SpawnerClass;

pub struct Spawner {
	player: u32,
	health: u32,
	required_food: u32,
}

impl BuildingClass for SpawnerClass {
	fn get_ref(&self) -> &'static BuildingClass { &SpawnerClass }
	fn get_required_terrain(&self) -> Option<Terrain> { None }
	fn get_build_item_cost(&self) -> &'static [ItemKind] {
		panic!("you should call get_build_item_cost() on Spawner!")
	}
	fn get_build_stamina_cost(&self) -> u32 {
		panic!("you should call get_build_stamina_cost() on Spawner!")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<Building> {
		panic!("you should never call build() on Spawner!")
	}
	fn get_name(&self) -> &'static str {
		"Spawner"
	}

	fn get_work_fn(&self) -> &'static fn(&mut World, Vector2u) {
		&WORK_FN
	}
}

impl Building for Spawner {
	fn as_any_mut(&mut self) -> &mut Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { SpawnerClass.get_ref() }
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
