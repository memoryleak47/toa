use crate::*;

pub struct Widget {
	pub pos: Vec2f,
	pub size: Vec2f,
	pub draw_type: DrawType,
	pub on_click: Vec<MenuCommand>,
	pub hotkey: Option<Key>,
}

pub enum DrawType {
	Color(Color),
	Texture(TextureId),
	Text(String),
}

impl App {
	pub fn draw_widget(&mut self, w: Widget) {
		match w.draw_type {
			DrawType::Color(color) => {
				let mut s = RectangleShape::new();
				s.set_fill_color(&color);
				s.set_position(vec2f_to_sfml(w.pos));
				s.set_size(vec2f_to_sfml(w.size));
				self.window.draw(&s);
			},
			DrawType::Texture(tid) => {
				let mut s = RectangleShape::with_texture(self.texture_state.get_texture(tid));
				s.set_position(vec2f_to_sfml(w.pos));
				s.set_size(vec2f_to_sfml(w.size));
				self.window.draw(&s);
			},
			DrawType::Text(string) => {
				let mut t = Text::new(&*string, &self.font, 15);
				t.set_position(vec2f_to_sfml(w.pos));
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

impl Widget {
	pub fn collides(&self, p: Vec2f) -> bool {
		p.x >= self.pos.x &&
		p.y >= self.pos.y &&
		p.x <= self.pos.x + self.size.x &&
		p.y <= self.pos.y + self.size.y
	}
}
