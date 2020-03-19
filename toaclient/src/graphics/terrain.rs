use toalib::world::terrainmap::Terrain;

use crate::graphics::{RawTextureId, TextureId, HasTexture};

impl HasTexture for Terrain {
	fn get_texture_id(&self) -> TextureId {
		match self {
			Terrain::GRASS => RawTextureId::GrassTerrain,
			Terrain::FOREST => RawTextureId::ForestTerrain,
			Terrain::STONE => RawTextureId::StoneTerrain,
			Terrain::IRON => RawTextureId::IronTerrain,
			Terrain::MOUNTAIN => RawTextureId::MountainTerrain,
			Terrain::MARSH => RawTextureId::MarshTerrain,
		}.into()
	}
}
