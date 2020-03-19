mod connect;
mod run;
mod tick;
mod render;
mod action;
mod marker;
mod menu;

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
use crate::app::menu::MenuState;

pub struct App {
	player_id: PlayerID,
	unit_mode: Option<UnitMode>, // None -> no unit focused
	focus_position: Vec2f,
	tilesize: f32, // zoom
	cursor: Pos,
	pending: Option<Action>,
	menu_state: MenuState,
	world: World,
	window: RenderWindow,
	input: Input,
	texture_state: TextureState,
	sound_state: SoundState,
	stream: Stream,
	font: Font,
}
