use toalib::world::terrainmap::Terrain;

use crate::graphics::TextureId;

pub fn get_texture_id(terrain: Terrain) -> TextureId {
	match terrain {
		Terrain::GRASS => TextureId::GrassTerrain,
		Terrain::FOREST => TextureId::ForestTerrain,
		Terrain::STONE => TextureId::StoneTerrain,
		Terrain::IRON => TextureId::IronTerrain,
	}
}
