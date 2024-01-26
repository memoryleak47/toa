use crate::*;

impl GameObject for Building {
    fn get_texture_id(&self) -> TextureId {
        match *self {
            Building::Spawner(ref spawner) => HuedTextureId {
                raw: RawTextureId::SpawnerBuilding,
                player_id: spawner.get_player_id(),
            }
            .into(),
            _ => self.get_class().get_texture_id(),
        }
    }
    fn get_relative_pos(&self) -> Vec2f {
        (0.).into()
    }
    fn get_size(&self) -> Vec2f {
        (1., 0.5).into()
    }
}

impl BuildingClass {
    pub fn get_texture_id(&self) -> TextureId {
        match *self {
            BuildingClass::Spawner => RawTextureId::SpawnerBuilding.into(),
            BuildingClass::Farm => RawTextureId::FarmBuilding.into(),
            BuildingClass::Camp => RawTextureId::CampBuilding.into(),
            BuildingClass::Sawmill => RawTextureId::SawmillBuilding.into(),
            BuildingClass::StoneMine => RawTextureId::StoneMineBuilding.into(),
            BuildingClass::IronMine => RawTextureId::IronMineBuilding.into(),
            BuildingClass::Workshop => RawTextureId::WorkshopBuilding.into(),
            BuildingClass::Castle => RawTextureId::CastleBuilding.into(),
            BuildingClass::WoodWall => RawTextureId::WoodWallBuilding.into(),
            BuildingClass::StoneWall => RawTextureId::StoneWallBuilding.into(),
            // BuildingClass::Street => RawTextureId::StreetBuilding.into(),
        }
    }
}
