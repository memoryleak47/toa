mod local;

pub use player::local::LocalPlayer;
pub use world::Command;
use view::View;
use world::World;

pub trait Player {
	fn fetch_command(&self, world: &World, &mut View) -> Option<Command>;
}
