use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::Vector2f;
use rand::{RngCore, thread_rng};

use view::View;

pub const TILESIZE: f32 = 20.;
pub const MAP_SIZE: usize = 16;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}

#[derive(Clone, Copy)]
pub enum Tile {
	GRASS,
	FOREST,
	STONE,
	IRON,
}

impl Tile {
	pub fn get_color(&self) -> Color {
		match self {
			Tile::GRASS => Color::rgb(50,150,50),
			Tile::FOREST => Color::rgb(0,50,0),
			Tile::STONE => Color::rgb(50,50,50),
			Tile::IRON => Color::rgb(150,150,150),
		}
	}
}

pub struct TileMap {
	tiles: [[Tile; MAP_SIZE]; MAP_SIZE] // tiles[x][y]
}

impl TileMap {
	pub fn gen() -> TileMap {
		let mut rng = thread_rng();

		let mut tiles = [[Tile::GRASS; MAP_SIZE]; MAP_SIZE];
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				let r = rng.next_u32();
				if r % 3 == 0 {
					tiles[x][y] = Tile::FOREST;
				} else if r % 7 == 0 {
					tiles[x][y] = Tile::STONE;
				} else if r % 11 == 0 {
					tiles[x][y] = Tile::IRON;
				}
			}
		}

		TileMap { tiles }
	}

	pub fn render(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				let posf = Vector2f::new(x as f32, y as f32);
				let size = window.size();

				let mut shape = RectangleShape::new();
				shape.set_fill_color(&self.tiles[x][y].get_color());
				shape.set_position((posf - view.focus_position) * TILESIZE + Vector2f::new(size.x as f32, size.y as f32) / 2.0);
				shape.set_size(TILESIZE_VEC());
				window.draw(&shape);
			}
		}
	}
}
