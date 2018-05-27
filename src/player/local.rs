use player::{Player, Command};
use view::View;
use world::World;

pub struct LocalPlayer {

}

impl LocalPlayer {
	pub fn new() -> LocalPlayer {
		LocalPlayer {}
	}
}

impl Player for LocalPlayer {
	fn fetch_command(&self, w: &World, view: &mut View) -> Option<Command> {
		return view.handle_action_keys(w);
	}
}
