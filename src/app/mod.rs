mod connect;
mod event;
#[cfg(feature = "fuzz")]
mod fuzz;
mod render;
mod run;
mod tick;

mod marker;
pub use marker::Marker;

use crate::*;

pub struct App {
    pub player_id: PlayerID,
    pub focus_position: Vec2f, // the tile position in the center of the screen
    pub tilesize: f32,         // zoom
    pub cursor: Pos,
    pub pending: Option<Vec<MenuCommand>>,
    pub menu_state: MenuState,
    pub world: World,
    pub animationmap: OptTileMap<Animation>,
    pub window: RenderWindow,
    pub texture_state: TextureState,
    pub sound_state: SoundState,
    pub stream: Stream,
    pub font: SfBox<Font>,
    pub window_grab: Option<Vec2f>,
    pub msg: String,
}
