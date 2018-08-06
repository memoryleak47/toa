pub mod default;

use sfml::system::{Vector2u, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable, Text, Font};

use world::{World, MAP_SIZE_X, MAP_SIZE_Y, TILESIZE, TILESIZE_VEC};
use misc::*;
use graphics::{TextureState, TextureId};

const MARKER_BORDER_SIZE: f32 = 5.;
lazy_static! {
	pub static ref CURSOR_COLOR: Color = Color::rgb(200, 150, 0);
	pub static ref TARGET_CURSOR_COLOR: Color = Color::rgb(200, 20, 20);
}

pub enum MarkerType {
	Transparent,
	Border
}

pub struct Marker {
	pub position: Vector2u,
	pub marker_type: MarkerType,
	pub color: &'static Color,
}

pub struct View {
	pub focus_position: Vector2f, // the tile in the center of the screen, in map coordinates
	pub markers: Vec<Marker>,
	pub player: u32, // important for the vision
	pub text: String
}

impl View {
	pub fn render(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		self.render_terrainmap(window, world, texture_state);
		self.render_buildingmap(window, world, texture_state);
		self.render_itemmap(window, world, texture_state);
		self.render_unitmap(window, world, texture_state);
		self.render_markers(window);

		self.render_hud(window, world);
	}

	fn render_hud(&self, window: &mut RenderWindow, world: &World) {
		let f = Font::from_file("/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();
		let t = Text::new(&self.text, &f, 15);
		window.draw(&t);
	}

	fn render_terrainmap(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let posf = Vector2f::new(x as f32, y as f32);

				let texture = texture_state.get_texture(world.terrainmap[x][y].get_texture_id());
				let mut shape = RectangleShape::with_texture(texture);
				shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
				shape.set_size(TILESIZE_VEC());
				window.draw(&shape);
			}
		}
	}

	fn render_buildingmap(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(building) = world.buildingmap[x][y]
							.as_ref()
							.map(|x| x.as_ref()) {
					let posf = Vector2f::new(x as f32, y as f32);

					let texture = texture_state.get_texture(building.get_texture_id());
					let mut shape = RectangleShape::with_texture(texture);
					shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
					shape.set_size(Vector2f::new(TILESIZE, TILESIZE/2.0));
					window.draw(&shape);
				}
			}
		}
	}

	fn render_itemmap(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if world.itemmap[x][y]
						.iter()
						.next()
						.is_some() {
					let posf = Vector2f::new(x as f32, y as f32);

					let texture = texture_state.get_texture(TextureId::Bag);
					let mut shape = RectangleShape::with_texture(texture);
					shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0 + Vector2f::new(0.0, 20.0));
					shape.set_size(Vector2f::new(10., 20.));
					window.draw(&shape);
				}
			}
		}
	}

	fn render_unitmap(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(ref unit) = world.unitmap[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);

					let texture_unit = texture_state.get_texture(TextureId::Unit);
					let mut shape = RectangleShape::with_texture(texture_unit);
					shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0 + Vector2f::new(10.0, 10.0));
					shape.set_size(Vector2f::new(20., 30.));
					window.draw(&shape);

					// TODO draw cloth
				}
			}
		}
	}

	fn render_markers(&self, window: &mut RenderWindow) {
		for marker in self.markers.iter() {
			marker.render(window, self);
		}
	}
}

impl Marker {
	fn render(&self, window: &mut RenderWindow, view: &View) {
		let halfscreen = Vector2f::new(window.size().x as f32, window.size().y as f32) / 2.0;
		let posf = vector_uf(self.position) * TILESIZE;

		let left_top = (posf - view.focus_position * TILESIZE) + halfscreen;
		let right_bot = left_top + TILESIZE_VEC();
		let (left, top) = (left_top.x, left_top.y);
		let (right, bot) = (right_bot.x, right_bot.y);

		let mut shape = RectangleShape::new();
		shape.set_fill_color(&self.get_effective_color());

		match self.marker_type {
			MarkerType::Transparent => {
				shape.set_position(left_top);
				shape.set_size(TILESIZE_VEC());
				window.draw(&shape);
			},
			MarkerType::Border => {
				// top
				shape.set_position(left_top);
				shape.set_size(Vector2f::new(TILESIZE as f32, MARKER_BORDER_SIZE));
				window.draw(&shape);

				// left
				shape.set_position(left_top);
				shape.set_size(Vector2f::new(MARKER_BORDER_SIZE, TILESIZE as f32));
				window.draw(&shape);

				// bot
				shape.set_position(Vector2f::new(left, bot - MARKER_BORDER_SIZE));
				shape.set_size(Vector2f::new(TILESIZE as f32, MARKER_BORDER_SIZE));
				window.draw(&shape);

				// right
				shape.set_position(Vector2f::new(right - MARKER_BORDER_SIZE, top));
				shape.set_size(Vector2f::new(MARKER_BORDER_SIZE, TILESIZE as f32));
				window.draw(&shape);
			},
		}
	}

	fn get_effective_color(&self) -> Color {
		match self.marker_type {
			MarkerType::Transparent => *self.color - Color::rgba(0, 0, 0, 155),
			MarkerType::Border => *self.color,
		}
	}
}
