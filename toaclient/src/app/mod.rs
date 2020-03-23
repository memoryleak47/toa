mod connect;
mod run;
mod tick;
mod render;
mod marker;
mod event;

use sfml::graphics::{RenderWindow, Font};

use toalib::world::World;
use toalib::vec::{Pos, Vec2f};
use toalib::team::PlayerID;
use toalib::net::Stream;

use crate::graphics::TextureState;
use crate::sound::SoundState;
use crate::menu::{MenuState, MenuCommand};

pub struct App {
	pub player_id: PlayerID,
	pub focus_position: Vec2f, // the tile position in the center of the screen
	pub tilesize: f32, // zoom
	pub cursor: Pos,
	pub pending: Vec<MenuCommand>,
	pub menu_state: MenuState,
	pub world: World,
	pub window: RenderWindow,
	pub texture_state: TextureState,
	pub sound_state: SoundState,
	pub stream: Stream,
	pub font: Font,
	pub window_grab: Option<Vec2f>,
}
