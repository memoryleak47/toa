use std::any::Any;

use crate::*;

mod spawner;
pub use spawner::*;

mod farm;
pub use farm::*;

mod camp;
pub use camp::*;

mod sawmill;
pub use sawmill::*;

mod stone_mine;
pub use stone_mine::*;

mod iron_mine;
pub use iron_mine::*;

mod workshop;
pub use workshop::*;

mod castle;
pub use castle::*;

mod wood_wall;
pub use wood_wall::*;

mod stone_wall;
pub use stone_wall::*;

// mod street;
// pub use street::*;

pub trait BuildingTrait {
    type Class: BuildingClassTrait + Sized;

    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_class(&self) -> BuildingClass;
    fn is_burnable(&self, _w: &World, _p: Pos) -> bool;
    fn is_workable(&self, _w: &World, _p: Pos) -> bool;
    fn damage(&mut self, damage: Damage) -> bool; // returns whether the building got destroyed

    // while this method is executed, the `self`-building is swapped out of the &mut World
    // `self` will only be placed back, if it wouldn't replace anything
    fn work(&mut self, _w: &mut World, _p: Pos);

    fn get_info_string(&self) -> String;
    fn is_blocking_against(&self, _pid: PlayerID) -> bool {
        false
    }
}

pub trait BuildingClassTrait {
    type Instance: BuildingTrait + Sized;

    fn get_build_property() -> Option<&'static BuildProperty>;
    fn get_name() -> &'static str;
    fn prevents_item_despawn() -> bool {
        false
    }
    fn reduces_walk_stamina() -> Option<u32> {
        None
    }
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
			pub fn is_burnable(&self, w: &World, p: Pos) -> bool		{ match self { $( Building::$x(a) => a.is_burnable(w, p) ),* } }
			pub fn is_workable(&self, w: &World, p: Pos) -> bool		{ match self { $( Building::$x(a) => a.is_workable(w, p) ),* } }
			pub fn damage(&mut self, damage: Damage) -> bool			{ match self { $( Building::$x(a) => a.damage(damage) ),* } }
			pub fn work(&mut self, w: &mut World, p: Pos)				{ match self { $( Building::$x(a) => a.work(w, p) ),* } }
			pub fn get_info_string(&self) -> String						{ match self { $( Building::$x(a) => a.get_info_string() ),* } }
			pub fn is_blocking_against(&self, pid: PlayerID) -> bool	{ match self { $( Building::$x(a) => a.is_blocking_against(pid) ),* } }

		}

		impl BuildingClass {
			pub fn get_build_property(&self) -> Option<&'static BuildProperty> { match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::get_build_property() ),* } }
			pub fn get_name(&self) -> &'static str						{ match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::get_name() ),* } }
			pub fn prevents_item_despawn(&self) -> bool					{ match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::prevents_item_despawn() ),* } }
			pub fn reduces_walk_stamina(&self) -> Option<u32>			{ match self { $( BuildingClass::$x => <$x as BuildingTrait>::Class::reduces_walk_stamina() ),* } }
		}
	};

}

setup!(Spawner, Farm, Camp, Sawmill, StoneMine, IronMine, Workshop, Castle, WoodWall, StoneWall); // deactivated: Street
