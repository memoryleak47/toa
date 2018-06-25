use sfml::graphics::Color;

use item::ItemKind;
use super::{BuildingClass, Building};
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref CONSTRUCTION_COLOR: Color = Color::rgb(90, 90, 40);
}
pub static CONSTRUCTION_CLASS: ConstructionClass = ConstructionClass;

pub struct ConstructionClass;

pub struct Construction {
	health: u32,
	invested_stamina: u32,
	build_class: &'static BuildingClass,
}

impl BuildingClass for ConstructionClass {
	fn get_required_terrain(&self) -> Option<Terrain> {
		panic!("get_required_terrain() should not be called on a Construction")
	}
	fn get_build_cost(&self) -> &'static [ItemKind] {
		panic!("get_build_cost() should not be called on a Construction")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<Building> {
		panic!("build() should not be called on a Construction")
	}
	fn get_name(&self) -> &'static str {
		"Construction"
	}
}

impl Building for Construction {
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { &CONSTRUCTION_CLASS }
	fn is_burnable(&self, unit: &Unit) -> bool { true }
	fn is_workable(&self, unit: &Unit) -> bool { true }
	fn get_color(&self) -> &'static Color {
		&CONSTRUCTION_COLOR
	}
}
