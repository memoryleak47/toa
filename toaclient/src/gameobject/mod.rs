use sfml::graphics::Color;

use crate::graphics::TextureId;
use toalib::vec::Vec2f;

pub mod terrain;
pub mod building;
pub mod item;
pub mod bag;
pub mod unit;

pub trait GameObject {
	fn get_texture_id(&self) -> TextureId;
	fn get_hue(&self) -> Option<Color> { None }
	fn get_relative_pos(&self) -> Vec2f;	// position (has to be added to its position); in tile-coordinates
	fn get_size(&self) -> Vec2f;			// in tile-coordinates
}
