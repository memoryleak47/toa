use std::any::Any;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item;
use item::ItemClass;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref FARM_COLOR: Color = Color::rgb(100, 100, 0);
	static ref WORK_FN: fn(&mut World, Vector2u) = |w, p| {
		let s = 40; // TODO un-hardcode
		let mut u = w.get_unit_mut(p).unwrap();
		u.stamina = u.stamina.saturating_sub(s);
		u.inventory.push(item::food::FoodClass.get_ref().build());
	};
}

pub struct FarmClass;

pub struct Farm {
	health: u32,
}

impl BuildingClass for FarmClass {
	fn get_ref(&self) -> &'static BuildingClass { &FarmClass }
	fn get_required_terrain(&self) -> Option<Terrain> { Some(Terrain::GRASS) }
	fn get_build_item_cost(&self) -> &'static [&'static ItemClass] {
		//&[ItemKind::Wood, ItemKind::Wood]
		&[] // TODO change back!
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
		&WORK_FN
	}
}

impl Building for Farm {
	fn as_any_mut(&mut self) -> &mut Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { FarmClass.get_ref() }
	fn is_burnable(&self, unit: &Unit) -> bool { true }
	fn is_workable(&self, unit: &Unit) -> bool { true }
	fn get_color(&self) -> &'static Color {
		&FARM_COLOR
	}
}
