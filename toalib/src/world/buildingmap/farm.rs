use std::any::Any;

use crate::vec::Vec2u;
use crate::item;
use crate::item::ItemClass;
use crate::item::wood::WoodClass;
use crate::world::buildingmap::{BuildingClass, Building};
use crate::world::World;
use crate::world::terrainmap::Terrain;
use crate::world::damage::Damage;

lazy_static! {
	static ref BUILD_ITEM_COST: [&'static dyn ItemClass; 2] = [WoodClass.get_ref(), WoodClass.get_ref()];
}

pub struct FarmClass;

#[derive(Clone)]
pub struct Farm {
	health: u32,
}

impl BuildingClass for FarmClass {
	fn get_ref(&self) -> &'static dyn BuildingClass { &FarmClass }
	fn get_required_terrain(&self) -> Option<Terrain> { Some(Terrain::GRASS) }
	fn get_build_item_cost(&self) -> &'static [&'static dyn ItemClass] {
		&BUILD_ITEM_COST[..]
	}
	fn get_build_stamina_cost(&self) -> u32 { 20 }
	fn get_height(&self) -> u32 { 0 }

	fn build(&self) -> Box<dyn Building> {
		Box::new(Farm { health: 100 })
	}
	fn get_name(&self) -> &'static str {
		"Farm"
	}
}

impl Building for Farm {
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> &'static dyn BuildingClass { FarmClass.get_ref() }
	fn is_burnable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vec2u) -> bool { true }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, w: &mut World, p: Vec2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.push(item::food::FoodClass.get_ref().build());
	}
}
