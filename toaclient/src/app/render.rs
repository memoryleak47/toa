use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable, Text};

use toalib::vec::{Pos, Vec2f, Vec2i};

use crate::graphics::{terrain, building, item, RawTextureId, HuedTextureId, TextureId};
use crate::vec_compat::*;
use crate::unit_mode::UnitMode;
use crate::app::App;

lazy_static! {
	pub static ref CURSOR_COLOR: Color = Color::rgb(200, 150, 0);
	pub static ref TARGET_CURSOR_COLOR: Color = Color::rgb(200, 20, 20);
}

enum MarkerType {
	Normal,
	Combat,
}

impl MarkerType {
	fn get_raw_texture_id(&self) -> RawTextureId {
		match self {
			MarkerType::Normal => RawTextureId::Cursor,
			MarkerType::Combat => RawTextureId::CombatCursor,
		}
	}
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
		for p in Pos::iter_all(){ 
			let posf = p.to_f();
			let size = Vec2f::with(1.);
			let texture_id = terrain::get_texture_id(*self.world.terrainmap.get(p));
			self.render_texture(posf, size, texture_id);
		}
	}

	fn render_buildingmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref building) = self.world.buildingmap.get(p) {
				let posf = p.to_f();
				let size = Vec2f::new(1., 0.5);
				let texture_id = building::get_texture_id(building);
				self.render_texture(posf, size, texture_id);
			}
		}
	}

	fn render_itemmap(&mut self) {
		for p in Pos::iter_all() {
			if self.world.itemmap.get(p)
					.iter()
					.next()
					.is_some() {
				let posf = p.to_f() + Vec2f::new(0., 0.5);
				let size = Vec2f::new(0.25, 0.5);
				let texture_id = RawTextureId::Bag.into();
				self.render_texture(posf, size, texture_id);
			}
		}
	}

	fn render_unitmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref u) = self.world.unitmap.get(p) {
				let player_id = u.owner;
				let posf = p.to_f() + Vec2f::with(0.25);
				let size = Vec2f::new(0.5, 0.75);
				let texture_id = RawTextureId::Unit.into();
				self.render_texture(posf, size, texture_id);

				let texture_id = HuedTextureId { raw: RawTextureId::UnitCloth, player_id }.into();
				self.render_texture(posf, size, texture_id);

				if let Some(ref main_item) = self.world.unitmap.get(p).unwrap().main_item {
					let pos = p.to_f() + Vec2f::new(0.5, 0.25);
					let size = Vec2f::new(0.5, 0.75);
					let texture_id = item::get_texture_id(main_item.get_class());
					self.render_texture(pos, size, texture_id);
				}
			}
		}
	}

	fn render_markers(&mut self) {
		self.render_marker(self.cursor, MarkerType::Normal); 

		let opt_tiles: Option<Vec<Vec2i>> = self.unit_mode.as_ref().and_then(|m| {
			match m {
				UnitMode::Attack { ref aim } => Some(aim.get_relative_tiles()),
				_ => None,
			}
		});

		let cursor: Pos = self.cursor;

		if let Some(tiles) = opt_tiles {
			for rel in tiles.into_iter() {
				if let Some(p) = cursor.map(|x| x + rel) {
					self.render_marker(p, MarkerType::Combat);
				}
			}
		}
	}

	fn render_marker(&mut self, pos: Pos, marker_type: MarkerType) {
		let posf = (*pos).map(|x| x as f32);
		let size = Vec2f::with(1.);
		let texture_id = marker_type.get_raw_texture_id().into();
		self.render_texture(posf, size, texture_id);
	}

	fn render_hud(&mut self) {
		let s = self.get_text();
		let t = Text::new(&*s, &self.font, 15);
		self.window.draw(&t);
	}

	fn get_text(&self) -> String {
		let pos = self.cursor;
		let terrain = self.world.terrainmap.get(pos);
		let building = self.world.buildingmap.get(pos);
		let unit = self.world.unitmap.get(pos).map(|x| x.get_info_string()).unwrap_or_else(|| "None".to_string());
		let inventory = self.world.itemmap.get(pos);

		let default = format!("Terrain: {:?}\nBuilding: {}\nUnit: {}\nItems: {}", terrain, building.map(|x| x.get_info_string()).unwrap_or("None".to_string()), unit, inventory.get_info_string());
		let action_infos = self.get_action_infos();

		let v: Vec<_> = action_infos.iter()
				.map(|x| x.get_text(&self.world))
				.collect();
		format!("{}\n{}", default, v.join("\n"))
	}

	#[allow(unused)]
	fn render_rectangle(&mut self, pos: Vec2f, size: Vec2f, color: Color) {
		let mut shape = RectangleShape::new();
		shape.set_fill_color(&color);

		render_shape(self.focus_position, &mut self.window, pos, size, shape, self.tilesize);
	}

	fn render_texture(&mut self, pos: Vec2f, size: Vec2f, texture_id: TextureId) {
		let shape = RectangleShape::with_texture(self.texture_state.get_texture(texture_id));
		render_shape(self.focus_position, &mut self.window, pos, size, shape, self.tilesize);
	}
}

fn render_shape(focus_position: Vec2f, window: &mut RenderWindow, pos: Vec2f, size: Vec2f, mut shape: RectangleShape, tilesize: f32) {
	let halfscreen = Vec2f::new(window.size().x as f32, window.size().y as f32) / 2.0;
	let posf = pos * tilesize;
	let left_top = (posf - focus_position * tilesize) + halfscreen;

	shape.set_position(vec2f_to_sfml(left_top));
	shape.set_size(vec2f_to_sfml(size * Vec2f::with(tilesize)));

	window.draw(&shape);
}
