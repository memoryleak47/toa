mod plane;
pub use plane::*;

use sfml::graphics::Color;

use toalib::vec::Vec2f;

use crate::app::App;
use crate::menu::{DrawCommand, MenuCommand};

pub trait Widget {
	fn get_draw_commands(&self, widget_size: Vec2f) -> Vec<DrawCommand>;
	fn get_position(&self, window_size: Vec2f) -> Vec2f;
	fn get_size(&self, window_size: Vec2f) -> Vec2f;
	fn on_click(&self) -> Option<MenuCommand> { None }
}
