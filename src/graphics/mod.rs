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
		let dir = ::misc::res_dir();
		let path_string = dir.to_str().unwrap();

		unsafe {
			let mut wrappers: [Texture; TEXTURE_COUNT] = mem::uninitialized();

			macro_rules! load {
				($a: expr, $b: expr) => {{
					let s = format!("{}/{}", path_string, $b);
					let texture = Texture::from_file(&s).unwrap();
					ptr::write(&mut wrappers[$a as usize], texture);
				}}
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
