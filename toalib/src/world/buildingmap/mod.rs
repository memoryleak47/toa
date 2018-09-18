pub mod spawner;
pub mod construction;
pub mod farm;


use std::any::Any;

use objekt;

use crate::vec::Vec2u;
use self::spawner::Spawner;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;
use crate::item::ItemClass;

lazy_static! {
	pub static ref BUILDABLE_CLASSES: [&'static dyn BuildingClass; 1] = [farm::FarmClass.get_ref()];
}

pub trait Building: objekt::Clone {
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn get_class(&self) -> &'static dyn BuildingClass;
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool;
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool;
	fn damage(&mut self, damage: Damage) -> bool; // returns whether the building got destroyed

	// while this method is executed, the `self`-building is swapped out of the &mut World
	// `self` will only be placed back, if it wouldn't replace anything
	fn work(&mut self, _w: &mut World, _p: Vec2u);
}

pub trait BuildingClass: Sync {
	fn get_ref(&self) -> &'static dyn BuildingClass;
	fn get_required_terrain(&self) -> Option<Terrain>;
	fn get_build_item_cost(&self) -> &'static [&'static dyn ItemClass];
	fn get_build_stamina_cost(&self) -> u32;
	fn get_height(&self) -> u32;
	fn build(&self) -> Box<dyn Building>;
	fn get_name(&self) -> &'static str;
}

pub fn new_buildingmap() -> [[Option<Box<dyn Building>>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut buildingmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	buildingmap[MAP_SIZE_X / 2][0] = Some(Spawner::new_boxed(0));
	buildingmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Spawner::new_boxed(1));

	buildingmap
}

impl World {
	pub fn get_building(&self, p: Vec2u) -> Option<&dyn Building> {
		self.buildingmap[p.x as usize][p.y as usize]
			.as_ref()
			.map(|x| x.as_ref())
	}

	#[allow(dead_code)]
	pub fn get_building_mut(&mut self, p: Vec2u) -> Option<&mut dyn Building> {
		// TODO make nicer! try map()
		if self.buildingmap[p.x as usize][p.y as usize].is_some() {
			Some(self.buildingmap[p.x as usize][p.y as usize].as_mut().unwrap().as_mut())
		} else {
			None
		}
	}

	pub fn set_building(&mut self, p: Vec2u, b: Option<Box<dyn Building>>) {
		self.buildingmap[p.x as usize][p.y as usize] = b;
	}
}
