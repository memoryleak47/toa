use toalib::world::terrainmap::Terrain;

use crate::graphics::{RawTextureId, TextureId};

pub fn get_texture_id(terrain: Terrain) -> TextureId {
	match terrain {
		Terrain::GRASS => RawTextureId::GrassTerrain,
		Terrain::FOREST => RawTextureId::ForestTerrain,
		Terrain::STONE => RawTextureId::StoneTerrain,
		Terrain::IRON => RawTextureId::IronTerrain,
		Terrain::MOUNTAIN => RawTextureId::MountainTerrain,
	}.into()
}
