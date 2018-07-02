use std::any::Any;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item;
use item::ItemClass;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

const REQUIRED_FOOD: u32 = 10;

lazy_static! {
	static ref TEAM_SPAWNER_COLOR: [Color; 2] = [Color::rgb(100, 0, 0), Color::rgb(0, 100, 0)];
	static ref WORK_FN: fn(&mut World, Vector2u) = |w, p| {

		use item::food::FoodClass;

		let u = w.get_unit_mut(p).unwrap();
		let item_vec: &mut Vec<_> = u.inventory.get_item_vec();
		for i in 0..REQUIRED_FOOD {
			let pos = item_vec.iter()
				.position(|x| x.get_class() == FoodClass.get_ref())
				.unwrap();
			item_vec.remove(pos);
		}
		let p2 = p + Vector2u::new(1, 0); // TODO check that this space is free!
		let new_unit = Unit::new(u.owner);
		w.set_unit(p2, Some(new_unit));
	};
}

pub struct SpawnerClass;

pub struct Spawner {
	player: u32,
	health: u32
}

impl BuildingClass for SpawnerClass {
	fn get_ref(&self) -> &'static BuildingClass { &SpawnerClass }
	fn get_required_terrain(&self) -> Option<Terrain> { None }
	fn get_build_item_cost(&self) -> &'static [&'static ItemClass] {
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
		use std::iter;

		let food = item::food::FoodClass.get_ref();
		let requirements: Vec<_> = iter::repeat(food)
			.take(REQUIRED_FOOD as usize)
			.collect();
		unit.inventory.contains_all(&requirements)
	}
	fn get_color(&self) -> &'static Color {
		&TEAM_SPAWNER_COLOR[self.player as usize]
	}
}

impl Spawner {
	pub fn new(player: u32) -> Spawner {
		Spawner { player, health: 100} // TODO un-hardcode
	}

	pub fn new_boxed(player: u32) -> Box<Building> {
		Box::new(Spawner::new(player))
	}
}
