use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable, Text, Font};

use toalib::misc::{vector_iu, vector_ui};
use toalib::config::{MAP_SIZE_X, MAP_SIZE_Y};
use toalib::vec::{Vec2u, Vec2f, Vec2i};

use crate::graphics::{terrain, building, item, TextureId};
use crate::vec_compat::*;
use crate::unit_mode::UnitMode;
use crate::app::App;
use crate::config::TILESIZE;

lazy_static! {
	pub static ref CURSOR_COLOR: Color = Color::rgb(200, 150, 0);
	pub static ref TARGET_CURSOR_COLOR: Color = Color::rgb(200, 20, 20);
}

enum MarkerType {
	Transparent,
	Border,
}

impl App {
	pub fn render(&mut self) {
		self.render_terrainmap();
		self.render_buildingmap();
		self.render_itemmap();
		self.render_unitmap();
		self.render_markers();
		self.render_hud();
	}

	fn render_terrainmap(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {

				let pos = Vec2f::new(x as f32, y as f32);
				let size = Vec2f::with(1.);
				let texture_id = terrain::get_texture_id(self.world.terrainmap[index2d!(x, y)]);
				self.render_texture(pos, size, texture_id);
			}
		}
	}

	fn render_buildingmap(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(ref building) = self.world.buildingmap[index2d!(x, y)].as_ref() {
					let pos = Vec2f::new(x as f32, y as f32);
					let size = Vec2f::new(1., 0.5);
					let texture_id = building::get_texture_id(building, &self.world.pool);
					self.render_texture(pos, size, texture_id);
				}
			}
		}
	}

	fn render_itemmap(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if self.world.itemmap[index2d!(x, y)]
						.iter()
						.next()
						.is_some() {
					let raw_pos = Vec2f::new(x as f32, y as f32);
					let pos = raw_pos + Vec2f::new(0., 0.5);
					let size = Vec2f::new(0.25, 0.5);
					let texture_id = TextureId::Bag;
					self.render_texture(pos, size, texture_id);
				}
			}
		}
	}

	fn render_unitmap(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if self.world.unitmap[index2d!(x, y)].is_some() {
					let raw_pos = Vec2f::new(x as f32, y as f32);
					let pos = raw_pos + Vec2f::with(0.25);
					let size = Vec2f::new(0.5, 0.75);
					let texture_id = TextureId::Unit;
					self.render_texture(pos, size, texture_id);

					// TODO draw cloth

					if let Some(ref main_item) = self.world.unitmap[index2d!(x, y)].as_ref().unwrap().main_item {
						let pos = raw_pos + Vec2f::new(0.5, 0.25);
						let size = Vec2f::new(0.5, 0.75);
						let texture_id = item::get_texture_id(main_item.get_class());
						self.render_texture(pos, size, texture_id);
					}
				}
			}
		}
	}

	fn render_markers(&mut self) {
		self.render_marker(self.cursor, &CURSOR_COLOR, MarkerType::Border); 

		let opt_tiles: Option<Vec<Vec2i>> = self.unit_mode.as_ref().and_then(|m| {
			match m {
				UnitMode::Attack { ref aim } => Some(aim.get_relative_tiles()),
				_ => None,
			}
		});

		let cursor = self.cursor;

		if let Some(tiles) = opt_tiles {
			for x in tiles.into_iter()
					.map(|x| x + vector_ui(cursor))
					.filter(|x| x.x >= 0 && x.y >= 0)
					.map(|x| vector_iu(x)) {

				self.render_marker(x, &TARGET_CURSOR_COLOR, MarkerType::Transparent);
			}
		}
	}

	fn render_marker(&mut self, pos: Vec2u, color: &Color, marker_type: MarkerType) {
		match marker_type {
			MarkerType::Transparent => {
				let pos = pos.map(|x| x as f32);
				let size = Vec2f::with(1.);
				let color = *color - Color::rgba(0, 0, 0, 155);
				self.render_rectangle(pos, size, color);
			},
			MarkerType::Border => {
				let pos = pos.map(|x| x as f32);
				let size = Vec2f::with(1.);
				let texture_id = TextureId::Cursor;
				self.render_texture(pos, size, texture_id);
			},
		}
	}

	fn render_hud(&mut self) {
		let f = Font::from_file("/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();
		let s = self.get_text();
		let t = Text::new(&*s, &f, 15);
		self.window.draw(&t);
	}

	fn get_text(&self) -> String {
		let pos = self.cursor;
		let terrain = self.world.get_terrain(pos);
		let building = self.world.get_building(pos);
		let unit = self.world.get_unit(pos).map(|x| x.get_info_string()).unwrap_or_else(|| "None".to_string());
		let inventory = self.world.get_inventory(pos);

		let default = format!("Terrain: {:?}\nBuilding: {}\nUnit: {}\nItems: {}", terrain, building.map(|x| x.get_class().get_name()).unwrap_or("None"), unit, inventory.get_info_string());
		let action_infos = self.get_action_infos();

		let v: Vec<_> = action_infos.iter()
				.map(|x| x.get_text())
				.collect();
		format!("{}\n{}", default, v.join("\n"))
	}

	fn render_rectangle(&mut self, pos: Vec2f, size: Vec2f, color: Color) {
		let mut shape = RectangleShape::new();
		shape.set_fill_color(&color);

		render_shape(self.focus_position, &mut self.window, pos, size, shape);
	}

	fn render_texture(&mut self, pos: Vec2f, size: Vec2f, texture_id: TextureId) {
		let shape = RectangleShape::with_texture(self.texture_state.get_texture(texture_id));
		render_shape(self.focus_position, &mut self.window, pos, size, shape);
	}
}

fn render_shape(focus_position: Vec2f, window: &mut RenderWindow, pos: Vec2f, size: Vec2f, mut shape: RectangleShape) {
	let halfscreen = Vec2f::new(window.size().x as f32, window.size().y as f32) / 2.0;
	let posf = pos * TILESIZE;
	let left_top = (posf - focus_position * TILESIZE) + halfscreen;

	shape.set_position(vec2f_to_sfml(left_top));
	shape.set_size(vec2f_to_sfml(size * Vec2f::with(TILESIZE)));

	window.draw(&shape);
}
