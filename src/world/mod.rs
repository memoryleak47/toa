mod tilemap;
mod buildingmap;
mod unitmap;

pub use self::tilemap::*;
pub use self::buildingmap::*;
pub use self::unitmap::*;

use sfml::graphics::{RenderTarget, RenderWindow, Text, Font};
use sfml::system::{Vector2f};

use input::Input;

use player::Player;
use view::View;
use command::Command;

pub const TILESIZE: f32 = 20.;
pub const MAP_SIZE: usize = 64;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}

// represents the current world situation
pub struct World {
	pub tilemap: [[Tile; MAP_SIZE]; MAP_SIZE],
	pub buildingmap: [[Option<Building>; MAP_SIZE]; MAP_SIZE],
	pub unitmap: [[Option<Unit>; MAP_SIZE]; MAP_SIZE],
	pub active_player: u8,
}

impl World {
	pub fn gen() -> World {
		World {
			tilemap: new_tilemap(),
			buildingmap: new_buildingmap(),
			unitmap: new_unitmap(),
			active_player: 0,
		}
	}

	pub fn render(&self, w: &mut RenderWindow, view: &View) {
		self.render_tilemap(w, view);
		self.render_buildingmap(w, view);
		self.render_unitmap(w, view);

		self.render_hud(w, view);
	}

	fn render_hud(&self, w: &mut RenderWindow, view: &View) {
		let f = Font::from_file("/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();

		let pos = view.marked_tile;

		let terrain = self.get_tile(pos);
		let building = self.get_building(pos);
		let unit = self.get_unit(pos);

		let t = Text::new(&format!("Active Player: {:?}\nTerrain: {:?}\nBuilding: {:?}\nUnit: {:?}", self.active_player, terrain, building, unit), &f, 30);
		w.draw(&t);
	}

	pub fn tick(&mut self, players: &[Box<Player>; 2], view: &mut View, input: &Input) {
		view.handle_basic_keys(input);

		if let Some(command) = players[self.active_player as usize].fetch_command(self, view, input) {
			self.exec(command, view);
		}
	}

	fn exec(&mut self, command: Command, view: &mut View) {
		match command {
			Command::Move { from, direction } => {
				let active_player = self.active_player;
				if self.try_move(from, direction, active_player) {
					view.marked_tile = direction.plus_vector(from);
				}
				// TODO do something in case this is an attack
			},
			Command::NextTurn => {
				self.active_player = 1 - self.active_player;

				if self.active_player == 0 {
					self.reset_turn();
				}
			},
		}
	}

	fn reset_turn(&mut self) {
		self.refill_stamina();
	}
}
