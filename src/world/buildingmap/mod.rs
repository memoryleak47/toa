pub mod spawner;
pub mod construction;
pub mod farm;

use objekt;

use std::any::Any;

use self::spawner::Spawner;

use sfml::system::Vector2u;

use graphics::TextureId;
use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::terrainmap::Terrain;
use item::ItemClass;

lazy_static! {
	pub static ref BUILDABLE_CLASSES: [&'static BuildingClass; 1] = [farm::FarmClass.get_ref()];
}

pub trait Building: objekt::Clone {
	fn get_texture_id(&self) -> TextureId;
	fn as_any_mut(&mut self) -> &mut Any;
	fn get_health(&self) -> u32;
	fn get_class(&self) -> &'static BuildingClass;
	fn is_burnable(&self, &World, Vector2u) -> bool;
	fn is_workable(&self, &World, Vector2u) -> bool;

	// while this method is executed, the `self`-building is swapped out of the &mut World
	// `self` will only be placed back, if it wouldn't replace anything
	fn work(&mut self, &mut World, Vector2u);
}

pub trait BuildingClass: Sync {
	fn get_ref(&self) -> &'static BuildingClass;
	fn get_required_terrain(&self) -> Option<Terrain>;
	fn get_build_item_cost(&self) -> &'static [&'static ItemClass];
	fn get_build_stamina_cost(&self) -> u32;
	fn get_height(&self) -> u32;
	fn build(&self) -> Box<Building>;
	fn get_name(&self) -> &'static str;
}

pub fn new_buildingmap() -> [[Option<Box<Building>>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut buildingmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	buildingmap[MAP_SIZE_X / 2][0] = Some(Spawner::new_boxed(0));
	buildingmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Spawner::new_boxed(1));

	buildingmap
}

impl World {
	pub fn get_building(&self, p: Vector2u) -> Option<&Building> {
		self.buildingmap[p.x as usize][p.y as usize]
			.as_ref()
			.map(|x| x.as_ref())
	}

	pub fn get_building_mut(&mut self, p: Vector2u) -> Option<&mut Building> {
		// TODO make nicer! try map()
		if self.buildingmap[p.x as usize][p.y as usize].is_some() {
			Some(self.buildingmap[p.x as usize][p.y as usize].as_mut().unwrap().as_mut())
		} else {
			None
		}
	}

	pub fn set_building(&mut self, p: Vector2u, b: Option<Box<Building>>) {
		self.buildingmap[p.x as usize][p.y as usize] = b;
	}
}
