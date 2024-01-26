use std::any::Any;

use crate::damage::Damage;
use crate::item::ItemClass;
use crate::vec::Pos;
use crate::world::buildingmap::{
    BuildProperty, Building, BuildingClass, BuildingClassTrait, BuildingTrait,
};
use crate::world::World;

lazy_static! {
    static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
        item_cost: &[
            ItemClass::Wood,
            ItemClass::Wood,
            ItemClass::Wood,
            ItemClass::Wood
        ],
        stamina_cost: 0,
        build: || Building::Workshop(Workshop { health: 100 }),
        required_terrain: None,
    };
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct WorkshopClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct Workshop {
    health: u32,
}

impl BuildingClassTrait for WorkshopClass {
    type Instance = Workshop;

    fn get_build_property() -> Option<&'static BuildProperty> {
        Some(&BUILD_PROPERTY)
    }
    fn get_name() -> &'static str {
        "Workshop"
    }
}

impl BuildingTrait for Workshop {
    type Class = WorkshopClass;

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_class(&self) -> BuildingClass {
        BuildingClass::Workshop
    }
    fn is_burnable(&self, _w: &World, _p: Pos) -> bool {
        true
    }
    fn is_workable(&self, _w: &World, _p: Pos) -> bool {
        false
    }
    fn damage(&mut self, damage: Damage) -> bool {
        self.health = self.health.saturating_sub(damage.0);
        self.health == 0
    }
    fn work(&mut self, _w: &mut World, _p: Pos) {
        panic!("can't work on workshop")
    }
    fn get_info_string(&self) -> String {
        format!("Workshop( health: {})", self.health)
    }
}
