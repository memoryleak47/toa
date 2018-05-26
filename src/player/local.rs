use player::Player;

pub struct LocalPlayer {

}

impl LocalPlayer {
	pub fn new() -> LocalPlayer {
		LocalPlayer {}
	}
}

impl Player for LocalPlayer {
}
