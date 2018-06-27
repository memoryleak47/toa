use std::any::Any;

use sfml::graphics::Color;
use sfml::system::Vector2u;

use item::ItemKind;
use super::{BuildingClass, Building};
use world::World;
use world::unitmap::Unit;
use world::terrainmap::Terrain;

lazy_static! {
	static ref CONSTRUCTION_COLOR: Color = Color::rgb(90, 90, 40);
	static ref WORK_FN: fn(&mut World, Vector2u) = |w, p| {
		let s = 10; // TODO un-hardcode
		let mut u = w.get_unit_mut(p).unwrap();
		u.stamina = u.stamina.saturating_sub(s);

		let mut construction: &mut Construction = w.get_building_mut(p)
				.unwrap()
				.as_any_mut()
				.downcast_mut()
				.unwrap();

		construction.invested_stamina += s;
		if construction.invested_stamina >= construction.build_class.get_build_stamina_cost() {
			let b = construction.build_class.build();
			w.set_building(p, Some(b));
		}
	};
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
	fn get_build_item_cost(&self) -> &'static [ItemKind] {
		panic!("get_build_item_cost() should not be called on a Construction")
	}
	fn get_build_stamina_cost(&self) -> u32 {
		panic!("get_build_stamina_cost() should not be called on a Construction")
	}
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<Building> {
		panic!("build() should not be called on a Construction")
	}
	fn get_name(&self) -> &'static str {
		"Construction"
	}
	fn get_work_fn(&self) -> &'static fn(&mut World, Vector2u) {
		&WORK_FN
	}
}

impl Building for Construction {
	fn as_any_mut(&mut self) -> &mut Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static BuildingClass { &CONSTRUCTION_CLASS }
	fn is_burnable(&self, unit: &Unit) -> bool { true }
	fn is_workable(&self, unit: &Unit) -> bool { true }
	fn get_color(&self) -> &'static Color {
		&CONSTRUCTION_COLOR
	}
}

impl Construction {
	pub fn new(class: &'static BuildingClass) -> Construction {
		Construction {
			health: 100, // TODO un-hardcode
			invested_stamina: 0,
			build_class: class,
		}
	}
}
