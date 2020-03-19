use crate::app::App;
use crate::menu::{Widget, Plane, DrawCommand, MenuCommand};

impl App {
	pub fn generate_widgets(&self) -> Vec<Box<dyn Widget>> {
		vec![Box::new(Plane)]
	}
}
