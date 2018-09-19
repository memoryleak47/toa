use std::any::Any;
use std::iter;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::world::buildingmap::{BuildingClass, Building};
use crate::world::World;
use crate::world::unitmap::Unit;
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;

const REQUIRED_FOOD: u32 = 10;

lazy_static! {
	static ref REQUIRED_FOOD_VEC: Vec<ItemClass> = {
		iter::repeat(ItemClass::Food)
			.take(REQUIRED_FOOD as usize)
			.collect()
	};
}

pub struct SpawnerClass;

#[derive(Clone)]
pub struct Spawner {
	player: u32,
	health: u32,
}

impl BuildingClass for SpawnerClass {
	fn get_ref(&self) -> &'static dyn BuildingClass { &SpawnerClass }
	fn get_required_terrain(&self) -> Option<Terrain> { None }
	fn get_build_item_cost(&self) -> &'static [ItemClass] {
		panic!("you should call get_build_item_cost() on Spawner!")
	}
	fn get_build_stamina_cost(&self) -> u32 {
		panic!("you should call get_build_stamina_cost() on Spawner!")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<dyn Building> {
		panic!("you should never call build() on Spawner!")
	}
	fn get_name(&self) -> &'static str {
		"Spawner"
	}
}

impl Building for Spawner {
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> &'static dyn BuildingClass { SpawnerClass.get_ref() }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { false }
	fn is_workable(&self, w: &World, p: Vec2u) -> bool {
		w.get_unit(p + Vec2u::new(1, 0)).is_none()
		&&
		w.get_unit(p)
			.filter(|u| u.inventory.contains_all(&REQUIRED_FOOD_VEC[..]))
			.is_some()
	}
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Vec2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.reduce(&REQUIRED_FOOD_VEC[..]);
		let p2 = p + Vec2u::new(1, 0);
		let new_unit = Unit::new(u.owner);
		w.set_unit(p2, Some(new_unit));
	}
}

impl Spawner {
	pub fn new(player: u32) -> Spawner {
		Spawner { player, health: 100} // TODO un-hardcode
	}

	pub fn new_boxed(player: u32) -> Box<dyn Building> {
		Box::new(Spawner::new(player))
	}
}
