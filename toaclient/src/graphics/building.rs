use toalib::world::buildingmap::Building;

use crate::graphics::{HuedTextureId, RawTextureId, TextureId};

pub fn get_texture_id(building: &Building) -> TextureId {
	match building {
		Building::Spawner(spawner) => {
			HuedTextureId {
				raw: RawTextureId::SpawnerBuilding,
				player_id: spawner.get_player_id()
			}.into()
		},
		Building::Farm(_) => RawTextureId::FarmBuilding.into(),
		Building::Camp(_) => RawTextureId::CampBuilding.into(),
		Building::Sawmill(_) => RawTextureId::SawmillBuilding.into(),
		Building::StoneMine(_) => RawTextureId::StoneMineBuilding.into(),
		Building::IronMine(_) => RawTextureId::IronMineBuilding.into(),
		Building::Workshop(_) => RawTextureId::WorkshopBuilding.into(),
		Building::Castle(_) => RawTextureId::CastleBuilding.into(),
		Building::StoneWall(_) => RawTextureId::StoneWallBuilding.into(),
	}
}

