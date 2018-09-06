use std::any::Any;

use sfml::system::Vector2u;

use graphics::TextureId;
use item;
use item::ItemClass;
use item::wood::WoodClass;
use super::{BuildingClass, Building};
use world::World;
use world::terrainmap::Terrain;

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
	fn get_texture_id(&self) -> TextureId { TextureId::FarmBuilding }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_health(&self) -> u32 { self.health }
	fn get_class(&self) -> &'static dyn BuildingClass { FarmClass.get_ref() }
	fn is_burnable(&self, _w: &World, _p: Vector2u) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Vector2u) -> bool { true }
	fn work(&mut self, w: &mut World, p: Vector2u) {
		let u = w.get_unit_mut(p).unwrap();
		u.inventory.push(item::food::FoodClass.get_ref().build());
	}
}
