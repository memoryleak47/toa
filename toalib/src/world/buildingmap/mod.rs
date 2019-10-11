mod spawner;
mod construction;
mod farm;
mod camp;
mod sawmill;
mod stone_mine;
mod iron_mine;
mod workshop;
mod castle;
mod stone_wall;

use std::any::Any;

use crate::vec::Vec2u;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::terrainmap::Terrain;
use crate::damage::Damage;
use crate::item::ItemClass;
use crate::team::PlayerID;

pub use self::spawner::Spawner;
use self::construction::Construction;
use self::farm::Farm;
use self::camp::Camp;
use self::sawmill::Sawmill;
use self::stone_mine::StoneMine;
use self::iron_mine::IronMine;
use self::workshop::Workshop;
use self::castle::Castle;
use self::stone_wall::StoneWall;

pub use self::spawner::new_spawner;
pub use self::construction::new_construction;

trait BuildingTrait {
	type Class: BuildingClassTrait + Sized;

	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn get_class(&self) -> BuildingClass;
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool;
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool;
	fn damage(&mut self, damage: Damage) -> bool; // returns whether the building got destroyed

	// while this method is executed, the `self`-building is swapped out of the &mut World
	// `self` will only be placed back, if it wouldn't replace anything
	fn work(&mut self, _w: &mut World, _p: Vec2u);

	fn get_info_string(&self) -> String;
	fn is_blocking_against(&self, _pid: PlayerID) -> bool { false }
}

trait BuildingClassTrait {
	type Instance: BuildingTrait + Sized;

	fn get_build_property() -> Option<&'static BuildProperty>;
	fn get_height() -> u32;
	fn get_name() -> &'static str;
}

#[derive(Clone)]
pub struct BuildProperty {
	pub item_cost: &'static [ItemClass],
	pub stamina_cost: u32,
	pub build: fn() -> Building,
	pub required_terrain: Option<Terrain>,
}

macro_rules! setup {
	($($x:ident),*) => {

		lazy_static! {
			pub static ref BUILDING_CLASSES: Vec<BuildingClass> = vec![ $( BuildingClass::$x),* ];
			pub static ref BUILDABLE_BUILDING_CLASSES: Vec<BuildingClass> = BUILDING_CLASSES.iter()
				.filter(|x| x.get_build_property().is_some())
				.cloned()
				.collect();
		}

		#[derive(Clone)]
		#[derive(Serialize, Deserialize)]
		pub enum Building {
			$(  $x($x)  ),*
		}

		#[derive(PartialEq, Eq, Copy, Clone)]
		#[derive(Serialize, Deserialize)]
		pub enum BuildingClass {
			$( $x ),*
		}

		impl Building {
			pub fn as_any_mut(&mut self) -> &mut dyn Any				{ match self { $( Building::$x(a) => a.as_any_mut() ),* } }
			pub fn get_class(&self) -> BuildingClass					{ match self { $( Building::$x(a) => a.get_class() ),* }  }
			pub fn is_burnable(&self, w: &World, p: Vec2u) -> bool		{ match self { $( Building::$x(a) => a.is_burnable(w, p) ),* } }
			pub fn is_workable(&self, w: &World, p: Vec2u) -> bool		{ match self { $( Building::$x(a) => a.is_workable(w, p) ),* } }
			pub fn damage(&mut self, damage: Damage) -> bool			{ match self { $( Building::$x(a) => a.damage(damage) ),* } }
			pub fn work(&mut self, w: &mut World, p: Vec2u)				{ match self { $( Building::$x(a) => a.work(w, p) ),* } }
			pub fn get_info_string(&self) -> String						{ match self { $( Building::$x(a) => a.get_info_string() ),* } }
			pub fn is_blocking_against(&self, pid: PlayerID) -> bool	{ match self { $( Building::$x(a) => a.is_blocking_against(pid) ),* } }
			
		}

		impl BuildingClass {
			pub fn get_build_property(&self) -> Option<&'static BuildProperty> { match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::get_build_property() ),* } }
			pub fn get_height(&self) -> u32								{ match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::get_height() ),* } }
			pub fn get_name(&self) -> &'static str						{ match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::get_name() ),* } }
		}
	};

}

setup!(Spawner, Construction, Farm, Camp, Sawmill, StoneMine, IronMine, Workshop, Castle, StoneWall);

pub fn new_buildingmap() -> Vec<Option<Building>> {
	let buildingmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	buildingmap
}

impl World {
	pub fn get_building(&self, p: Vec2u) -> Option<&Building> {
		self.buildingmap[index2d!(p.x, p.y)]
			.as_ref()
	}

	#[allow(dead_code)]
	pub fn get_building_mut(&mut self, p: Vec2u) -> Option<&mut Building> {
		self.buildingmap[index2d!(p.x, p.y)]
			.as_mut()
	}

	pub fn set_building(&mut self, p: Vec2u, b: Option<Building>) {
		self.buildingmap[index2d!(p.x, p.y)] = b;
	}
}
