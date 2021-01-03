use crate::*;

// sadly, I can't use a normal method, due to borrowing issues
macro_rules! draw {
	($self:expr, $p:expr, $go:expr) => {
		{
			let halfscreen = $self.window_size() / 2.0;
			let pos = $p.to_f() + $go.get_relative_pos();
			let size = $go.get_size();

			let tilesize = $self.tilesize;

			let texture = $self.texture_state.get_texture($go.get_texture_id());
			let texsize = texture.size();

			let mut sprite = Sprite::with_texture(texture);
			if let Some(color) = $go.get_hue() {
				sprite.set_color(&color);
			}

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
		self.render_animations();
		self.render_menu();
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
				if let Some(item) = u.equipped_item() {
					draw!(self, p, item);
				}
			}
		}
	}

	fn render_markers(&mut self) {
		draw!(self, self.cursor, &Marker::Normal);
		match self.menu_state {
			MenuState::Attack(weapon_id) => {
				let v = self.get_world_mouse() - self.cursor.to_f();
				let u = self.world.unitmap.get(self.cursor).unwrap();
				let rel_tiles = weapon_id.map(|i| u.inventory.get(i)).aim(v);
				for t in rel_tiles {
					if let Some(t) = self.cursor.map(|x| x + t) {
						draw!(self, t, &Marker::Combat);
					}
				}
			}
			MenuState::DropItem(_) => {
				draw!(self, self.cursor, &Marker::ItemDrop);
				for d in Direction::iter() {
					if let Some(c) = self.cursor.map(|p| p + *d) {
						draw!(self, c, &Marker::ItemDrop);
					}
				}
			}
			MenuState::SpawnUnit => {
				for d in Direction::iter() {
					if let Some(c) = self.cursor.map(|p| p + *d) {
						draw!(self, c, &Marker::SpawnUnit);
					}
				}
			}
			_ => ()
		}
	}

	fn render_animations(&mut self) {
		for t in Pos::iter_all() {
			if let Some(a) = self.animationmap.get(t) {
				draw!(self, t, &a.kind);
			}
		}
	}

	pub fn window_size(&self) -> Vec2f {
		Vec2f::new(self.window.size().x as f32, self.window.size().y as f32)
	}
}
