use sfml::graphics::Color;

use crate::app::App;
use crate::menu::{Widget, MenuCommand};

impl App {
	pub fn generate_widgets(&self) -> Vec<Widget> {
		let ws = self.window_size();
		vec![
			Widget { // plane
				pos: (0.).into(),
				size: ws * (0.3, 1.),
				draw_type: Color::rgb(100, 100, 100).into(),
				on_click: None,
			},
		]
	}
}
