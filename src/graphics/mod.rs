use std::ptr;
use std::mem;

use sfml::graphics::Texture;

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum TextureId {
	GrassTile,
	ForestTile,
	StoneTile,
	IronTile,
}

const TEXTURE_COUNT: usize = 4;

pub struct TextureState {
	wrappers: [Texture; TEXTURE_COUNT],
}

impl TextureState {
	pub fn new() -> TextureState {
		unsafe {
			let mut wrappers: [Texture; TEXTURE_COUNT] = mem::uninitialized();

			macro_rules! load {
				($a: expr, $b: expr) => {
					ptr::write(&mut wrappers[$a as usize], Texture::from_file(&format!("res/{}", $b)).unwrap());
				}
			}

			load!(TextureId::GrassTile, "tile/grass.png");
			load!(TextureId::ForestTile, "tile/forest.png");
			load!(TextureId::StoneTile, "tile/stone.png");
			load!(TextureId::IronTile, "tile/iron.png");

			TextureState { wrappers }
		}
	}

	pub fn get_texture(&self, id: TextureId) -> &Texture {
		&self.wrappers[id as usize]
	}
}
