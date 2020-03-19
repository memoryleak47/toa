use toalib::vec::Vec2f;

use crate::graphics::{TextureId, RawTextureId, GameObject};

pub struct Bag;

impl GameObject for Bag {
	fn get_texture_id(&self) -> TextureId { RawTextureId::Bag.into() }
	fn get_relative_pos(&self) -> Vec2f { (0., 0.5).into() }
	fn get_size(&self) -> Vec2f { (0.25, 0.5).into() }
}
