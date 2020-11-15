use crate::*;

impl GameObject for Terrain {
	fn get_texture_id(&self) -> TextureId {
		match self {
			Terrain::GRASS => RawTextureId::GrassTerrain,
			Terrain::PLAINS => RawTextureId::PlainsTerrain,
			Terrain::FOREST => RawTextureId::ForestTerrain,
			Terrain::STONE => RawTextureId::StoneTerrain,
			Terrain::IRON => RawTextureId::IronTerrain,
			Terrain::MOUNTAIN => RawTextureId::MountainTerrain,
			Terrain::MARSH => RawTextureId::MarshTerrain,
		}.into()
	}
	fn get_relative_pos(&self) -> Vec2f { <_>::from(0.) }
	fn get_size(&self) -> Vec2f { <_>::from(1.) }
}
