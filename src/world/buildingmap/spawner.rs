use std::any::Any;

use crate::damage::Damage;
use crate::team::PlayerID;
use crate::vec::Pos;
use crate::world::buildingmap::{
    BuildProperty, Building, BuildingClass, BuildingClassTrait, BuildingTrait,
};
use crate::world::World;

// see tick_spawners()!

const MAX_HEALTH: u32 = 100;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct SpawnerClass;

#[derive(Clone, Serialize, Deserialize)]
pub struct Spawner {
    player: PlayerID,
    health: u32,
}

impl BuildingClassTrait for SpawnerClass {
    type Instance = Spawner;

    fn get_build_property() -> Option<&'static BuildProperty> {
        None
    }
    fn get_name() -> &'static str {
        "Spawner"
    }
    fn prevents_item_despawn() -> bool {
        true
    }
}

impl BuildingTrait for Spawner {
    type Class = SpawnerClass;

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_class(&self) -> BuildingClass {
        BuildingClass::Spawner
    }
    fn is_burnable(&self, _w: &World, _p: Pos) -> bool {
        false
    }
    fn is_workable(&self, _w: &World, _p: Pos) -> bool {
        false
    }
    fn damage(&mut self, damage: Damage) -> bool {
        self.health = self.health.saturating_sub(damage.0);
        self.health == 0
    }
    fn work(&mut self, _w: &mut World, _p: Pos) {}
    fn get_info_string(&self) -> String {
        format!("Spawner( health: {}, player: {})", self.health, self.player)
    }
    fn is_blocking_against(&self, pid: PlayerID) -> bool {
        pid != self.player
    }
}

impl Spawner {
    pub fn get_player_id(&self) -> PlayerID {
        self.player
    }
}

pub fn new_spawner(player: PlayerID) -> Building {
    Building::Spawner(Spawner {
        player,
        health: MAX_HEALTH,
    })
}
