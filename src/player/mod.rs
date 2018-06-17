mod local;
mod ai;

pub use player::local::LocalPlayer;
pub use player::ai::AiPlayer;
pub use command::Command;

use input::Input;
use view::View;
use world::World;

pub trait Player {
	fn tick(&mut self, world: &World, input: &Input) -> Option<Command>;
	fn get_view(&self, w: &World) -> View;

	fn turn_start(&mut self) {}
	fn turn_end(&mut self) {}
}
