mod control;

use sfml::system::{Vector2u, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, CircleShape, Shape, Color, Transformable, Text, Font};

use world::{World, MAP_SIZE_X, MAP_SIZE_Y, TILESIZE, TILESIZE_VEC};
use misc::*;
use graphics::TextureState;

const MARK_BORDER_SIZE: u32 = 5;

#[allow(non_snake_case)]
fn MAIN_CURSOR_COLOR() -> Color { Color::rgb(200, 150, 0) }

#[allow(non_snake_case)]
fn SECOND_CURSOR_COLOR() -> Color { Color::rgb(200, 0, 0) }

pub struct View {
	pub focus_position: Vector2f, // the tile in the center of the screen, in map coordinates
	pub main_cursor: Vector2u, // the marked tile position
	pub second_cursor: Option<Vector2u>, // the optional second marked tile position
	pub marked_tiles: Vec<Vector2u>, // a set of slightly transparently marked tiles
	pub player: u32 // important for the vision
}

impl View {
	pub fn new(player: u32) -> View {
		View {
			focus_position: Vector2f::new(MAP_SIZE_X as f32 / 2., MAP_SIZE_Y as f32 / 2.),
			main_cursor: Vector2u::new(0, 0),
			second_cursor: None,
			marked_tiles: Vec::new(),
			player
		}
	}

	pub fn move_cursor(&mut self, direction: Direction) {
		self.main_cursor = direction.plus_vector(self.main_cursor);
	}

	fn render_marker(&self, window: &mut RenderWindow, color: &Color, size: u32, position: Vector2u) {
		let posf = Vector2f::new(position.x as f32 * TILESIZE, position.y as f32 * TILESIZE);

		let halfscreen = Vector2f::new(window.size().x as f32, window.size().y as f32) / 2.0;

		let mut shape = RectangleShape::new();
		shape.set_fill_color(color);

		// top
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen);
		shape.set_size(Vector2f::new(TILESIZE as f32, size as f32));
		window.draw(&shape);

		// left
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen);
		shape.set_size(Vector2f::new(size as f32, TILESIZE as f32));
		window.draw(&shape);

		// bottom
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen + Vector2f::new(0., TILESIZE - size as f32));
		shape.set_size(Vector2f::new(TILESIZE as f32, size as f32));
		window.draw(&shape);

		// right
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen + Vector2f::new(TILESIZE - size as f32, 0.));
		shape.set_size(Vector2f::new(size as f32, TILESIZE as f32));
		window.draw(&shape);
	}

	pub fn render(&self, window: &mut RenderWindow, world: &World, texture_state: &TextureState) {
		self.render_terrainmap(window, world, texture_state);
		self.render_buildingmap(window, world);
		self.render_unitmap(window, world);

		self.render_hud(window, world);

		self.render_marker(window, &MAIN_CURSOR_COLOR(), MARK_BORDER_SIZE, self.main_cursor);
		if let Some(cursor) = self.second_cursor {
			self.render_marker(window, &SECOND_CURSOR_COLOR(), MARK_BORDER_SIZE, cursor);
		}
		// TODO render marked_tiles
	}

	fn render_hud(&self, window: &mut RenderWindow, world: &World) {
		let f = Font::from_file("/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();

		let pos = self.main_cursor;

		let terrain = world.get_terrain(pos);
		let building = world.get_building(pos);
		let unit = world.get_unit(pos);

		let t = Text::new(&format!("Active Player: {:?}\nTerrain: {:?}\nBuilding: {:?}\nUnit: {:?}", world.active_player, terrain, building, unit), &f, 30);
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

	fn render_unitmap(&self, window: &mut RenderWindow, world: &World) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(unit) = world.unitmap[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);

					let mut shape = CircleShape::new(TILESIZE / 2.0, 200);
					shape.set_fill_color(&unit.get_color());
					shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
					window.draw(&shape);
				}
			}
		}
	}

	fn render_buildingmap(&self, window: &mut RenderWindow, world: &World) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(building) = world.buildingmap[x][y].as_ref() {
					let posf = Vector2f::new(x as f32, y as f32);

					let mut shape = RectangleShape::new();
					shape.set_fill_color(&building.kind.get_color());
					shape.set_position((posf - self.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
					shape.set_size(TILESIZE_VEC());
					window.draw(&shape);
				}
			}
		}
	}
}
