mod local;
mod ai;

pub use self::local::LocalPlayer;
pub use self::ai::AiPlayer;
pub use crate::command::Command;

use crate::input::Input;
use crate::view::View;
use crate::world::World;

pub trait Player {
	fn tick(&mut self, world: &World, input: &Input) -> Option<Command>;
	fn get_view(&self, w: &World) -> View;

	fn turn_start(&mut self) {}
	fn turn_end(&mut self) {}
}
