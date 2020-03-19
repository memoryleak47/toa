mod connect;
mod run;
mod tick;
mod render;
mod action;
mod marker;

use sfml::graphics::{RenderWindow, Font};

use toalib::world::World;
use toalib::vec::{Pos, Vec2f};
use toalib::team::PlayerID;
use toalib::net::Stream;

use crate::app::action::Action;
use crate::unit_mode::UnitMode;
use crate::input::Input;
use crate::graphics::TextureState;
use crate::sound::SoundState;
use crate::menu::MenuState;

pub struct App {
	pub player_id: PlayerID,
	pub unit_mode: Option<UnitMode>, // None -> no unit focused
	pub focus_position: Vec2f,
	pub tilesize: f32, // zoom
	pub cursor: Pos,
	pub pending: Option<Action>,
	pub menu_state: MenuState,
	pub world: World,
	pub window: RenderWindow,
	pub input: Input,
	pub texture_state: TextureState,
	pub sound_state: SoundState,
	pub stream: Stream,
	pub font: Font,
}
