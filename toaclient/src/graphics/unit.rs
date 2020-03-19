use sfml::graphics::Color;

use toalib::vec::Vec2f;
use toalib::team::PlayerID;
pub use toalib::world::Unit;

use crate::graphics::{HuedTextureId, RawTextureId, TextureId, GameObject};

pub struct Cloth(pub PlayerID);

const RELATIVE_POS: Vec2f = Vec2f::new(0.25, 0.25);
const SIZE: Vec2f = Vec2f::new(0.5, 0.755);

const NO_STAMINA_ALPHA: u8 = 170;

impl GameObject for Unit {
	fn get_texture_id(&self) -> TextureId { RawTextureId::Unit.into() }
	fn get_hue(&self) -> Option<Color> {
		Some(Color::rgba(255, 255, 255, NO_STAMINA_ALPHA))
			.filter(|_| self.stamina <= 0)
	}
	fn get_relative_pos(&self) -> Vec2f { RELATIVE_POS }
	fn get_size(&self) -> Vec2f { SIZE }
}

impl GameObject for Cloth {
	fn get_texture_id(&self) -> TextureId {
		HuedTextureId { raw: RawTextureId::UnitCloth, player_id: self.0 }.into()
	}
	fn get_relative_pos(&self) -> Vec2f { RELATIVE_POS }
	fn get_size(&self) -> Vec2f { SIZE }
}
