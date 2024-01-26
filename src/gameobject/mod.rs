use crate::*;

mod terrain;
pub use terrain::*;

mod building;
pub use building::*;

mod item;
pub use item::*;

mod bag;
pub use bag::*;

mod unit;
pub use unit::*;

pub trait GameObject {
    fn get_texture_id(&self) -> TextureId;
    fn get_hue(&self) -> Option<Color> {
        None
    }
    fn get_relative_pos(&self) -> Vec2f; // position (has to be added to its position); in tile-coordinates
    fn get_size(&self) -> Vec2f; // in tile-coordinates
}
