use sfml::system::Vector2f;
use sfml::window::Key;

use input::Input;
use view::View;

impl View {
	// This implements the view-functionality, which is used if the active-player is no LocalPlayer, it should be called in *Player::tick()
	pub fn tick_default(&mut self, input: &Input) {
		if let Some(direction) = input.move_direction() {
			if input.is_pressed(Key::LControl) || input.is_pressed(Key::RControl) {
				let v = direction.to_vector();
				self.focus_position += Vector2f::new(v.x as f32, v.y as f32);
			} else {
				self.move_cursor(direction);
			}
		}
	}
}
