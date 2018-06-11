mod local;

pub use player::local::LocalPlayer;
pub use command::Command;

use input::Input;
use view::View;
use world::World;

pub trait Player {
	fn tick(&mut self, world: &World, &mut View, input: &Input) -> Option<Command>;

	fn turn_start(&mut self) {}
	fn turn_end(&mut self) {}

	fn uses_view(&self) -> bool;
}
