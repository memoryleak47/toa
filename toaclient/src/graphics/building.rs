use toalib::world::buildingmap::Building;
use toalib::vec::Vec2f;

use crate::graphics::{HuedTextureId, RawTextureId, TextureId, HasTexture, GameObject};

impl HasTexture for Building {
	fn get_texture_id(&self) -> TextureId {
		match *self {
			Building::Spawner(ref spawner) => {
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
			Building::WoodWall(_) => RawTextureId::WoodWallBuilding.into(),
			Building::StoneWall(_) => RawTextureId::StoneWallBuilding.into(),
			Building::Street(_) => RawTextureId::StreetBuilding.into(),
		}
	}
}

impl GameObject for Building {
	fn get_relative_pos(&self) -> Vec2f { (0.).into() }
	fn get_size(&self) -> Vec2f { (1.).into() }
}
