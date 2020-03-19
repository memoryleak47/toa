use toalib::vec::Vec2f;

use crate::graphics::{RawTextureId, TextureId, HasTexture, GameObject};

pub enum Marker {
	Normal,
	Combat,
}

impl HasTexture for Marker {
	fn get_texture_id(&self) -> TextureId {
		match self {
			Marker::Normal => RawTextureId::Cursor.into(),
			Marker::Combat => RawTextureId::CombatCursor.into(),
		}
	}
}

impl GameObject for Marker {
	fn get_relative_pos(&self) -> Vec2f { <_>::from(0.) }
	fn get_size(&self) -> Vec2f { <_>::from(1.) }
}


