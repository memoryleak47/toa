use crate::app::App;

use sfml::graphics::{RenderTarget, Color};

impl App {
	pub fn run(&mut self) {
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				self.handle_event(event);
			}

			self.tick();
			self.render();

			self.window.display();
			self.window.clear(&Color::rgb(0, 0, 0));

			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}
}
