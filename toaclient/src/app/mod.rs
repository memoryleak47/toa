mod connect;
mod run;
mod tick;
mod render;
mod action;

use sfml::graphics::RenderWindow;

use toalib::world::World;
use toalib::vec::{Vec2u, Vec2f};
use toalib::team::PlayerID;
use toalib::net::Stream;

use crate::app::action::Action;
use crate::unit_mode::UnitMode;
use crate::input::Input;
use crate::graphics::TextureState;
use crate::sound::SoundState;

pub struct App {
	player_id: PlayerID,
	unit_mode: Option<UnitMode>, // None -> no unit focused
	focus_position: Vec2f,
	cursor: Vec2u,
	pending: Option<Action>,
	world: World,
	window: RenderWindow,
	input: Input,
	texture_state: TextureState,
	sound_state: SoundState,
	stream: Stream,
}
