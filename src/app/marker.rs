use crate::*;

// This could be normal structs
pub enum Marker {
	Normal,
	Combat,
	ItemDrop,
}

impl GameObject for Marker {
	fn get_texture_id(&self) -> TextureId {
		match self {
			Marker::Normal => RawTextureId::Cursor.into(),
			Marker::Combat => RawTextureId::CombatCursor.into(),
			Marker::ItemDrop => RawTextureId::ItemDropCursor.into(),
		}
	}
	fn get_relative_pos(&self) -> Vec2f { <_>::from(0.) }
	fn get_size(&self) -> Vec2f { <_>::from(1.) }
}
