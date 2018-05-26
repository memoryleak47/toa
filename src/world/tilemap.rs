use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::Vector2f;

const TILESIZE: f32 = 20.;
const MAP_SIZE: usize = 64;

fn TILESIZE_VEC() -> Vector2f {
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
			Tile::GRASS => Color::rgb(0,200,0),
			Tile::FOREST => Color::rgb(0,50,0),
			Tile::STONE => Color::rgb(70,70,70),
			Tile::IRON => Color::rgb(100,100,100),
		}
	}
}

pub struct TileMap {
	tiles: [[Tile; MAP_SIZE]; MAP_SIZE] // tiles[x][y]
}

impl TileMap {
	pub fn gen() -> TileMap {
		let mut tiles = [[Tile::GRASS; MAP_SIZE]; MAP_SIZE];
		for (x, y) in (0..MAP_SIZE).zip(0..MAP_SIZE) {
			if x % 2 == 0 && y % 2 == 0 {
				tiles[x][y] = Tile::FOREST;
			}
			if x % 4 == 0 && y % 4 == 0 {
				tiles[x][y] = Tile::STONE;
			}

			if x % 8 == 0 && y % 4 == 0 {
				tiles[x][y] = Tile::STONE;
			}
		}

		TileMap { tiles }
	}

	pub fn render(&self, window: &mut RenderWindow) {
		for (x, y) in (0..MAP_SIZE).zip(0..MAP_SIZE) {
			let mut shape = RectangleShape::new();
			shape.set_fill_color(&self.tiles[x][y].get_color());
			shape.set_position(Vector2f::new(x as f32 * TILESIZE, y as f32 * TILESIZE));
			shape.set_size(TILESIZE_VEC());
			window.draw(&shape);
		}
	}
}
