use std::any::Any;

use crate::*;

lazy_static! {
	static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
		item_cost: &[ItemClass::Stone, ItemClass::Stone, ItemClass::Stone, ItemClass::Stone],
		stamina_cost: 0,
		build: || Building::Castle(Castle { health: 400 }),
		required_terrain: None,
	};
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct CastleClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Castle {
	health: u32,
}

impl BuildingClassTrait for CastleClass {
	type Instance = Castle;

	fn get_build_property() -> Option<&'static BuildProperty> { Some(&BUILD_PROPERTY) }
	fn get_name() -> &'static str {
		"Castle"
	}
}

impl BuildingTrait for Castle {
	type Class = CastleClass;

	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn get_class(&self) -> BuildingClass { BuildingClass::Castle }
	fn is_burnable(&self, _w: &World, _p: Pos) -> bool { true }
	fn is_workable(&self, _w: &World, _p: Pos) -> bool { false }
	fn damage(&mut self, damage: Damage) -> bool {
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
	fn work(&mut self, _w: &mut World, _p: Pos) {
		panic!("can't work on castle");
	}
	fn get_info_string(&self) -> String {
		format!("Castle( health: {})", self.health)
	}


}
