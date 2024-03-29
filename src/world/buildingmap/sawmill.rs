use std::any::Any;

use crate::*;

lazy_static! {
    static ref BUILD_PROPERTY: BuildProperty = BuildProperty {
        item_cost: &[
            ItemClass::Wood,
            ItemClass::Wood,
            ItemClass::Wood,
            ItemClass::Wood
        ],
        stamina_cost: 0,
        build: || Building::Sawmill(Sawmill { health: 100 }),
        required_terrain: Some(Terrain::FOREST),
    };
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct SawmillClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sawmill {
    health: u32,
}

impl BuildingClassTrait for SawmillClass {
    type Instance = Sawmill;

    fn get_build_property() -> Option<&'static BuildProperty> {
        Some(&BUILD_PROPERTY)
    }
    fn get_name() -> &'static str {
        "Sawmill"
    }
}

impl BuildingTrait for Sawmill {
    type Class = SawmillClass;

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_class(&self) -> BuildingClass {
        BuildingClass::Sawmill
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
    fn work(&mut self, _: &mut World, _: Pos) {
        panic!("you cannot work at sawmill!")
    }
    fn get_info_string(&self) -> String {
        format!("Sawmill( health: {})", self.health)
    }
}
