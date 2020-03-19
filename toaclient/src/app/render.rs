use sfml::graphics::{RenderTarget, Sprite, Color, Transformable, Text};

use toalib::vec::{Pos, Vec2f, Vec2i};
use toalib::world::Building;

use crate::graphics::{RawTextureId, HuedTextureId, TextureId, HasTexture};
use crate::vec_compat::*;
use crate::unit_mode::UnitMode;
use crate::app::App;

static NO_STAMINA_ALPHA: u8 = 170;

lazy_static! {
	pub static ref CURSOR_COLOR: Color = Color::rgb(200, 150, 0);
	pub static ref TARGET_CURSOR_COLOR: Color = Color::rgb(200, 20, 20);
}

enum MarkerType {
	Normal,
	Combat,
}

impl HasTexture for MarkerType {
	fn get_texture_id(&self) -> TextureId {
		match self {
			MarkerType::Normal => RawTextureId::Cursor.into(),
			MarkerType::Combat => RawTextureId::CombatCursor.into(),
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
			let size = (1.).into();
			let t = *self.world.terrainmap.get(p);
			self.render_texture(posf, size, &t);
		}
	}

	fn render_buildingmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref building) = self.world.buildingmap.get(p) {
				let posf = p.to_f();
				let size = Vec2f::new(1., 0.5);
				let building: Building = (*building).clone(); // TODO there is a better way!
				self.render_texture(posf, size, &building);
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
				let texture_id: TextureId = RawTextureId::Bag.into();
				self.render_texture(posf, size, &texture_id);
			}
		}
	}

	fn render_unitmap(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref u) = self.world.unitmap.get(p) {
				let player_id = u.owner;
				let posf = p.to_f() + 0.25;
				let size = (0.5, 0.75).into();
				let texture_id: TextureId = RawTextureId::Unit.into();
				let color = if u.stamina <= 0 { Some(Color::rgba(255, 255, 255, NO_STAMINA_ALPHA)) } else { None };
				self.render_colored_texture(posf, size, &texture_id, color);

				let texture_id: TextureId = HuedTextureId { raw: RawTextureId::UnitCloth, player_id }.into();
				self.render_colored_texture(posf, size, &texture_id, color);

				if let Some(ref main_item) = self.world.unitmap.get(p).unwrap().main_item {
					let pos = p.to_f() + Vec2f::new(0.5, 0.25);
					let size = Vec2f::new(0.5, 0.75);
					let class = main_item.get_class();
					self.render_colored_texture(pos, size, &class, color); // TODO this has to work a better way
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
		let size: Vec2f = (1.).into();
		self.render_texture(posf, size, &marker_type);
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

	fn render_colored_texture<H: HasTexture>(&mut self, pos: Vec2f, size: Vec2f, h: &H, color: Option<Color>) {
		let tilesize = self.tilesize;

		let texture = self.texture_state.get_texture(h.get_texture_id());
		let texsize = texture.size();

		let mut sprite = Sprite::with_texture(texture);
		if let Some(color) = color {
			sprite.set_color(&color);
		}

		let halfscreen = Vec2f::new(self.window.size().x as f32, self.window.size().y as f32) / 2.0;
		let posf = pos * tilesize;
		let left_top = (posf - self.focus_position * tilesize) + halfscreen;

		sprite.set_position(vec2f_to_sfml(left_top));
		let xscale = (size.x * tilesize) / (texsize.x as f32);
		let yscale = (size.y * tilesize) / (texsize.y as f32);
		sprite.set_scale(vec2f_to_sfml(Vec2f::new(xscale, yscale)));

		self.window.draw(&sprite);
	}

	fn render_texture<H: HasTexture>(&mut self, pos: Vec2f, size: Vec2f, h: &H) {
		self.render_colored_texture(pos, size, h, None);
	}
}
