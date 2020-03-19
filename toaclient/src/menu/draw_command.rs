use sfml::graphics::{Color, RectangleShape, RenderTarget, Transformable, Shape, Text};

use toalib::vec::Vec2f;
use crate::graphics::TextureId;
use crate::app::App;
use crate::vec_compat::*;

pub enum DrawType {
	Color(Color),
	Texture(TextureId),
	Text(String),
}

pub struct DrawCommand {
	pub pos: Vec2f,
	pub size: Vec2f,
	pub draw_type: DrawType,
}

impl App {
	pub fn execute_draw_command(&mut self, c: DrawCommand) {
		match c.draw_type {
			DrawType::Color(color) => {
				let mut s = RectangleShape::new();
				s.set_fill_color(&color);
				s.set_position(vec2f_to_sfml(c.pos));
				s.set_size(vec2f_to_sfml(c.size));
				self.window.draw(&s);
			},
			DrawType::Texture(tid) => {
				let mut s = RectangleShape::with_texture(self.texture_state.get_texture(tid));;
				s.set_position(vec2f_to_sfml(c.pos));
				s.set_size(vec2f_to_sfml(c.size));
				self.window.draw(&s);
			},
			DrawType::Text(string) => {
				let mut t = Text::new(&*string, &self.font, 15);
				t.set_position(vec2f_to_sfml(c.pos));
				self.window.draw(&t);
				// TODO make sure text stays in c.size-box
			},
		}
	}
}

impl From<Color> for DrawType {
	fn from(c: Color) -> DrawType { DrawType::Color(c) }
}

impl From<TextureId> for DrawType {
	fn from(tid: TextureId) -> DrawType { DrawType::Texture(tid) }
}

impl From<String> for DrawType {
	fn from(s: String) -> DrawType { DrawType::Text(s) }
}
