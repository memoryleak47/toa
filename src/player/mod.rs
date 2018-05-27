mod local;

pub use player::local::LocalPlayer;
pub use world::Command;

use input::Input;
use view::View;
use world::World;

pub trait Player {
	fn fetch_command(&self, world: &World, &mut View, input: &Input) -> Option<Command>;
}
