use std::any::Any;
use std::iter;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item::ItemClass;
use item::food::FoodClass;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

const REQUIRED_FOOD: u32 = 10;

lazy_static! {
	static ref TEAM_SPAWNER_COLOR: [Color; 2] = [Color::rgb(100, 0, 0), Color::rgb(0, 100, 0)];
	static ref REQUIRED_FOOD_VEC: Vec<&'static ItemClass> = {
		let food = FoodClass.get_ref();
		iter::repeat(food)
			.take(REQUIRED_FOOD as usize)
			.collect()
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
}

impl Building for Spawner {
	fn as_any_mut(&mut self) -> &mut Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { SpawnerClass.get_ref() }
	fn is_burnable(&self, w: &World, p: Vector2u) -> bool { false }
	fn is_workable(&self, w: &World, p: Vector2u) -> bool {
		w.get_unit(p + Vector2u::new(1, 0)).is_none()
		&&
		w.get_unit(p)
			.filter(|u| u.inventory.contains_all(&REQUIRED_FOOD_VEC[..]))
			.is_some()
	}
	fn get_color(&self) -> &'static Color {
		&TEAM_SPAWNER_COLOR[self.player as usize]
	}
	fn work(&mut self, w: &mut World, p: Vector2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.reduce(&REQUIRED_FOOD_VEC[..]);
		let p2 = p + Vector2u::new(1, 0);
		let new_unit = Unit::new(u.owner);
		w.set_unit(p2, Some(new_unit));
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
