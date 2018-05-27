use player::{Player, Command};
use view::View;

pub struct LocalPlayer {

}

impl LocalPlayer {
	pub fn new() -> LocalPlayer {
		LocalPlayer {}
	}
}

impl Player for LocalPlayer {
	fn fetch_command(&self, view: &mut View) -> Option<Command> {
		view.handle_action_keys();

		None
	}
}
