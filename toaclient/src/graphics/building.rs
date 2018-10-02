use toalib::world::buildingmap::Building;
use toalib::team::{Team, PlayerPool};

use crate::graphics::TextureId;

pub fn get_texture_id(building: &Building, player_pool: &PlayerPool) -> TextureId {
	match building {
		Building::Spawner(spawner) => {
			match player_pool.get_team_of(spawner.get_player_id()) {
				Team::Red => TextureId::SpawnerRedBuilding,
				Team::Blue => TextureId::SpawnerBlueBuilding,
			}
		},
		Building::Construction(_) => TextureId::ConstructionBuilding,
		Building::Farm(_) => TextureId::FarmBuilding,
		Building::Camp(_) => TextureId::CampBuilding,
		Building::Sawmill(_) => TextureId::SawmillBuilding,
		Building::StoneMine(_) => TextureId::StoneMineBuilding,
		Building::IronMine(_) => TextureId::IronMineBuilding,
		Building::Workshop(_) => TextureId::WorkshopBuilding,
	}
}

