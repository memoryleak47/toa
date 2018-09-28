use crate::app::App;

use sfml::graphics::{RenderTarget, Color};
use sfml::window::Event;

impl App {
	pub fn run(&mut self) {
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				if event == Event::Closed {
					self.window.close();
				}
			}

			self.tick();
			self.render();
			self.render();

			self.window.display();
			self.window.clear(&Color::rgb(0, 0, 0));
		}
		
	}
}
