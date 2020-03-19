use sfml::graphics::Color;

use toalib::vec::Vec2f;

use crate::app::App;
use crate::menu::{Widget, DrawCommand, MenuCommand};

pub struct Plane;

impl Widget for Plane {
	fn get_draw_commands(&self, widget_size: Vec2f) -> Vec<DrawCommand> {
		vec![DrawCommand {
			pos: (0.).into(),
			size: widget_size,
			draw_type: Color::rgb(100, 100, 100).into(),
		}]
	}

	fn get_position(&self, _: Vec2f) -> Vec2f {
		(0.).into()
	}

	fn get_size(&self, window_size: Vec2f) -> Vec2f {
		window_size * (0.25, 1.)
	}
}
