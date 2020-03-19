use sfml::graphics::{RenderTarget, Sprite, Transformable, Text};

use toalib::vec::{Pos, Vec2f};

use crate::graphics::GameObject;
use crate::graphics::{bag::Bag, unit::Cloth};
use crate::vec_compat::*;
use crate::unit_mode::UnitMode;
use crate::app::App;
use crate::app::marker::Marker;


// sadly, I can't use a normal method, due to borrowing issues
macro_rules! draw {
	($self:expr, $p:expr, $go:expr) => {
		{
			let pos = $p.to_f() + $go.get_relative_pos();
			let size = $go.get_size();

			let tilesize = $self.tilesize;

			let texture = $self.texture_state.get_texture($go.get_texture_id());
			let texsize = texture.size();

			let mut sprite = Sprite::with_texture(texture);
			if let Some(color) = $go.get_hue() {
				sprite.set_color(&color);
			}

			let halfscreen = Vec2f::new($self.window.size().x as f32, $self.window.size().y as f32) / 2.0;
			let posf = pos * tilesize;
			let left_top = (posf - $self.focus_position * tilesize) + halfscreen;

			sprite.set_position(vec2f_to_sfml(left_top));
			let xscale = (size.x * tilesize) / (texsize.x as f32);
			let yscale = (size.y * tilesize) / (texsize.y as f32);
			sprite.set_scale(vec2f_to_sfml(Vec2f::new(xscale, yscale)));

			$self.window.draw(&sprite);
		}
	};
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
		for p in Pos::iter_all() {
			draw!(self, p, self.world.terrainmap.get(p));
		}
	}

	fn render_buildingmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(building) = self.world.buildingmap.get(p) {
				draw!(self, p, building);
			}
		}
	}

	fn render_itemmap(&mut self) {
		for p in Pos::iter_all() {
			if self.world.itemmap.get(p)
					.iter()
					.next()
					.is_some() {
				draw!(self, p, Bag);
			}
		}
	}

	fn render_unitmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(u) = self.world.unitmap.get(p) {
				draw!(self, p, u);
				draw!(self, p, &Cloth(u.owner));
				if let Some(ref main_item) = u.main_item {
					draw!(self, p, main_item);
				}
			}
		}
	}

	fn render_markers(&mut self) {
		draw!(self, self.cursor, &Marker::Normal);
		if let Some(UnitMode::Attack { ref aim }) = self.unit_mode {
			for rel in aim.get_relative_tiles() {
				if let Some(p) = self.cursor.map(|x| x + rel) {
					draw!(self, p, Marker::Combat);
				}
			}
		}
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
}
