use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable, Text, Font};

use toalib::misc::{vector_uf, vector_iu, vector_ui};
use toalib::world::World;
use toalib::config::{MAP_SIZE_X, MAP_SIZE_Y, TILESIZE, TILESIZE_VEC};
use toalib::vec::{Vec2u, Vec2f};

use crate::graphics::{terrain, building, TextureState, TextureId};
use crate::vec_compat::*;
use crate::controller::{Controller, UnitMode};

const MARKER_BORDER_SIZE: f32 = 5.;
lazy_static! {
	pub static ref CURSOR_COLOR: Color = Color::rgb(200, 150, 0);
	pub static ref TARGET_CURSOR_COLOR: Color = Color::rgb(200, 20, 20);
}

enum MarkerType {
	Transparent,
	Border,
}

pub fn render(window: &mut RenderWindow, world: &World, texture_state: &TextureState, controller: &Controller) {
	render_terrainmap(window, world, texture_state, controller);
	render_buildingmap(window, world, texture_state, controller);
	render_itemmap(window, world, texture_state, controller);
	render_unitmap(window, world, texture_state, controller);
	render_markers(window, controller);

	render_hud(window, world, controller);
}

fn render_hud(window: &mut RenderWindow, world: &World, controller: &Controller) {
	let f = Font::from_file("/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();
	let s = controller.get_text(world);
	let t = Text::new(&*s, &f, 15);
	window.draw(&t);
}

fn render_terrainmap(window: &mut RenderWindow, world: &World, texture_state: &TextureState, controller: &Controller) {
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			let posf = Vec2f::new(x as f32, y as f32);

			let texture_id = terrain::get_texture_id(world.terrainmap[index2d!(x, y)]);
			let texture = texture_state.get_texture(texture_id);
			let mut shape = RectangleShape::with_texture(texture);
			shape.set_position(vec2f_to_sfml((posf - controller.focus_position) * TILESIZE + vector_uf(vector2u_to_toa(window.size())) / 2.0));
			shape.set_size(vec2f_to_sfml(TILESIZE_VEC()));
			window.draw(&shape);
		}
	}
}

fn render_buildingmap(window: &mut RenderWindow, world: &World, texture_state: &TextureState, controller: &Controller) {
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			if let Some(ref building) = world.buildingmap[index2d!(x, y)]
						.as_ref() {
				let posf = Vec2f::new(x as f32, y as f32);

				let texture_id = building::get_texture_id(building, &world.pool);
				let texture = texture_state.get_texture(texture_id);
				let mut shape = RectangleShape::with_texture(texture);
				shape.set_position(vec2f_to_sfml((posf - controller.focus_position) * TILESIZE + vector_uf(vector2u_to_toa(window.size())) / 2.0));
				shape.set_size(vec2f_to_sfml(Vec2f::new(TILESIZE, TILESIZE/2.0)));
				window.draw(&shape);
			}
		}
	}
}

fn render_itemmap(window: &mut RenderWindow, world: &World, texture_state: &TextureState, controller: &Controller) {
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			if world.itemmap[index2d!(x, y)]
					.iter()
					.next()
					.is_some() {
				let posf = Vec2f::new(x as f32, y as f32);

				let texture = texture_state.get_texture(TextureId::Bag);
				let mut shape = RectangleShape::with_texture(texture);
				shape.set_position(vec2f_to_sfml((posf - controller.focus_position) * TILESIZE + vector_uf(vector2u_to_toa(window.size())) / 2.0 + Vec2f::new(0.0, 20.0)));
				shape.set_size(vec2f_to_sfml(Vec2f::new(10., 20.)));
				window.draw(&shape);
			}
		}
	}
}

fn render_unitmap(window: &mut RenderWindow, world: &World, texture_state: &TextureState, controller: &Controller) {
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			if let Some(ref _unit) = world.unitmap[index2d!(x, y)] {
				let posf = Vec2f::new(x as f32, y as f32);

				let texture_unit = texture_state.get_texture(TextureId::Unit);
				let mut shape = RectangleShape::with_texture(texture_unit);
				shape.set_position(vec2f_to_sfml((posf - controller.focus_position) * TILESIZE + vector_uf(vector2u_to_toa(window.size())) / 2.0 + Vec2f::new(10.0, 10.0)));
				shape.set_size(vec2f_to_sfml(Vec2f::new(20., 30.)));
				window.draw(&shape);

				// TODO draw cloth
			}
		}
	}
}

fn render_markers(window: &mut RenderWindow, controller: &Controller) {
	render_marker(
		window,
		controller.cursor,
		controller,
		&CURSOR_COLOR,
		MarkerType::Border,
	);

	if let Some(UnitMode::Attack { ref aim }) = controller.unit_mode {
		for x in aim.get_relative_tiles().iter()
				.map(|x| *x + vector_ui(controller.cursor))
				.filter(|x| x.x >= 0 && x.y >= 0)
				.map(|x| vector_iu(x)) {
			render_marker(
				window,
				x,
				controller,
				&TARGET_CURSOR_COLOR,
				MarkerType::Transparent,
			);
		}
	}
}

fn render_marker(window: &mut RenderWindow, pos: Vec2u, controller: &Controller, color: &Color, marker_type: MarkerType) {
	let halfscreen = Vec2f::new(window.size().x as f32, window.size().y as f32) / 2.0;
	let posf = vector_uf(pos) * TILESIZE;

	let left_top = (posf - controller.focus_position * TILESIZE) + halfscreen;
	let right_bot = left_top + TILESIZE_VEC();
	let (left, top) = (left_top.x, left_top.y);
	let (right, bot) = (right_bot.x, right_bot.y);

	let mut shape = RectangleShape::new();
	let mut color = *color;
	if let MarkerType::Transparent = marker_type {
		color -= Color::rgba(0, 0, 0, 155);
	}
	shape.set_fill_color(&color);

	match marker_type {
		MarkerType::Transparent => {
			shape.set_position(vec2f_to_sfml(left_top));
			shape.set_size(vec2f_to_sfml(TILESIZE_VEC()));
			window.draw(&shape);
		},
		MarkerType::Border => {
			// top
			shape.set_position(vec2f_to_sfml(left_top));
			shape.set_size(vec2f_to_sfml(Vec2f::new(TILESIZE as f32, MARKER_BORDER_SIZE)));
			window.draw(&shape);

			// left
			shape.set_position(vec2f_to_sfml(left_top));
			shape.set_size(vec2f_to_sfml(Vec2f::new(MARKER_BORDER_SIZE, TILESIZE as f32)));
			window.draw(&shape);

			// bot
			shape.set_position(vec2f_to_sfml(Vec2f::new(left, bot - MARKER_BORDER_SIZE)));
			shape.set_size(vec2f_to_sfml(Vec2f::new(TILESIZE as f32, MARKER_BORDER_SIZE)));
			window.draw(&shape);

			// right
			shape.set_position(vec2f_to_sfml(Vec2f::new(right - MARKER_BORDER_SIZE, top)));
			shape.set_size(vec2f_to_sfml(Vec2f::new(MARKER_BORDER_SIZE, TILESIZE as f32)));
			window.draw(&shape);
		},
	}
}
