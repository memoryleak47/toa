pub mod spawner;
pub mod construction;
pub mod farm;

use std::any::Any;

use self::spawner::Spawner;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use world::terrainmap::Terrain;
use world::unitmap::Unit;
use item::ItemKind;
pub static BUILDABLE_CLASSES: [&BuildingClass; 1] = [&farm::FARM_CLASS];

pub trait Building {
	fn as_any_mut(&mut self) -> &mut Any;
	fn get_health(&self) -> u32;
	fn get_class(&self) -> &'static BuildingClass;
	fn is_burnable(&self, unit: &Unit) -> bool;
	fn is_workable(&self, unit: &Unit) -> bool;
	fn get_color(&self) -> &'static Color;
}

pub trait BuildingClass: Sync {
	fn get_required_terrain(&self) -> Option<Terrain>;
	fn get_build_item_cost(&self) -> &'static [ItemKind];
	fn get_build_stamina_cost(&self) -> u32;
	fn get_height(&self) -> u32;
	fn build(&self) -> Box<Building>;
	fn get_name(&self) -> &'static str;
	fn get_work_fn(&self) -> &'static fn(&mut World, Vector2u);
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
