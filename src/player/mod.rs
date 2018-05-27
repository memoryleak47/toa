mod local;

pub use player::local::LocalPlayer;
pub use world::Command;
use view::View;

pub trait Player {
	fn fetch_command(&self, &mut View) -> Option<Command>;
}
