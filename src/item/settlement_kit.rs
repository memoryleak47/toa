use crate::vec::Pos;
use crate::item::{Item, ItemClass, ItemTrait, ItemClassTrait};
use crate::world::World;
use crate::damage::Damage;
use crate::world::buildingmap::new_spawner;

lazy_static! {
	static ref RECIPE: [ItemClass; 8] = [ItemClass::Stone, ItemClass::Stone, ItemClass::Stone, ItemClass::Stone, ItemClass::Wood, ItemClass::Wood, ItemClass::Wood, ItemClass::Wood];
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SettlementKitClass;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct SettlementKit;

impl ItemClassTrait for SettlementKitClass {
	type Instance = SettlementKit;
	
	fn get_name() -> &'static str { "SettlementKit" }
	fn get_weight() -> u32 { 100 }
	fn build() -> Item {
		Item::SettlementKit(SettlementKit)
	}
	fn get_recipe() -> Option<&'static [ItemClass]> { Some(&RECIPE[..]) }
	fn stateless() -> bool { false }
}

impl ItemTrait for SettlementKit {
	type Class = SettlementKitClass;

	fn get_class(&self) -> ItemClass {
		ItemClass::SettlementKit
	}
	fn damage(&mut self, _: Damage) -> bool { true }
	fn is_execable(&self, p: Pos, w: &World) -> bool {
		w.buildingmap.get(p).is_none() && !w.terrainmap.get(p).prevents_building()
	}
	fn exec(&self, p: Pos, w: &mut World) {
		let s = new_spawner(w.unitmap.get(p).unwrap().owner);
        w.buildingmap.set(p, Some(s));
	}
}
