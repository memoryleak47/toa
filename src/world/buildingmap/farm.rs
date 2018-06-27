use std::any::Any;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item::ItemKind;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref FARM_COLOR: Color = Color::rgb(100, 100, 0);
}
pub static FARM_CLASS: FarmClass = FarmClass;

pub struct FarmClass;

pub struct Farm {
	health: u32,
}

impl BuildingClass for FarmClass {
	fn get_required_terrain(&self) -> Option<Terrain> { Some(Terrain::GRASS) }
	fn get_build_item_cost(&self) -> &'static [ItemKind] {
		&[ItemKind::Wood, ItemKind::Wood]
	}
	fn get_build_stamina_cost(&self) -> u32 { 20 }
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<Building> {
		Box::new(Farm { health: 100 })
	}
	fn get_name(&self) -> &'static str {
		"Farm"
	}
	fn get_work_fn(&self) -> &'static fn(&mut World, Vector2u) {
		unimplemented!()
	}
}

impl Building for Farm {
	fn as_any_mut(&mut self) -> &mut Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { &FARM_CLASS }
	fn is_burnable(&self, unit: &Unit) -> bool { true }
	fn is_workable(&self, unit: &Unit) -> bool { true }
	fn get_color(&self) -> &'static Color {
		&FARM_COLOR
	}
}
