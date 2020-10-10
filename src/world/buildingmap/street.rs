use std::any::Any;

use crate::*;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Stone],
		stamina_cost: 0,
		build: || Building::Street(Street { health: 5 }),
		required_terrain: None,
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct StreetClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Street {
	health: u32,
}

impl BuildingClassTrait for StreetClass {
	type Instance = Street;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"Street"
	}
	fn reduces_walk_stamina() -> Option<u32> { Some(5) }
}

impl BuildingTrait for Street {
	type Class = StreetClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Street }
	fn is_burnable(&self, _w: &World, _p: Pos) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Pos) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _: &mut World, _: Pos) { panic!("can't work on street") }
	fn get_info_string(&self) -> String {
		format!("Street( health: {})", self.health)
	}
}
