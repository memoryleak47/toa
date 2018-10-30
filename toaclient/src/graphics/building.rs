use toalib::world::buildingmap::Building;

use crate::graphics::TextureId;

pub fn get_texture_id(building: &Building) -> TextureId {
	match building {
		Building::Spawner(spawner) => {
			match spawner.get_player_id().0 {
				0 => TextureId::SpawnerRedBuilding, // TODO fix this!
				_ => TextureId::SpawnerBlueBuilding,
			}
		},
		Building::Construction(_) => TextureId::ConstructionBuilding,
		Building::Farm(_) => TextureId::FarmBuilding,
		Building::Camp(_) => TextureId::CampBuilding,
		Building::Sawmill(_) => TextureId::SawmillBuilding,
		Building::StoneMine(_) => TextureId::StoneMineBuilding,
		Building::IronMine(_) => TextureId::IronMineBuilding,
		Building::Workshop(_) => TextureId::WorkshopBuilding,
		Building::Castle(_) => TextureId::CastleBuilding,
	}
}

