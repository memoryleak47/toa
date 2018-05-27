mod tilemap;
mod buildingmap;
mod unitmap;

pub use self::tilemap::*;
pub use self::buildingmap::*;
pub use self::unitmap::*;

use sfml::graphics::{RenderTarget, RenderWindow, Text, Font};
use sfml::system::{Vector2u, Vector2i};
use std::cmp::{min, max};

pub use world::tilemap::{TILESIZE, MAP_SIZE, TILESIZE_VEC};

use input::Input;

use player::Player;
use view::View;

#[derive(Copy, Clone)]
pub enum Direction {
	Up, Left, Down, Right
}

pub fn crop_vector(v: Vector2i) -> Vector2u {
	Vector2u::new(
		max(0, min(MAP_SIZE as i32 - 1, v.x)) as u32,
		max(0, min(MAP_SIZE as i32 - 1, v.y)) as u32,
	)
}

impl Direction {
	pub fn to_vector(&self) -> Vector2i {
		match self {
			Direction::Up => Vector2i::new(0, -1),
			Direction::Left => Vector2i::new(-1, 0),
			Direction::Down => Vector2i::new(0, 1),
			Direction::Right => Vector2i::new(1, 0),
		}
	}

	pub fn plus_vector(&self, p: Vector2u) -> Vector2u {
		let v = self.to_vector();
		crop_vector(Vector2i::new(p.x as i32 + v.x, p.y as i32 + v.y))
	}
}

pub enum Command {
	Move { from: Vector2u, direction: Direction },
	NextTurn,
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
