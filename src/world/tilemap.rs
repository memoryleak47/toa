use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};
use rand::{RngCore, thread_rng};

use view::View;
use world::{World, TILESIZE, TILESIZE_VEC, MAP_SIZE};
use misc::vector_uf;

#[derive(Clone, Copy, Debug)]
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

pub fn new_tilemap() -> [[Tile; MAP_SIZE]; MAP_SIZE] {
	let mut rng = thread_rng();

	let mut tilemap = [[Tile::GRASS; MAP_SIZE]; MAP_SIZE];
	for x in 0..MAP_SIZE {
		for y in 0..MAP_SIZE {
			let r = rng.next_u32();
			if r % 3 == 0 {
				tilemap[x][y] = Tile::FOREST;
			} else if r % 7 == 0 {
				tilemap[x][y] = Tile::STONE;
			} else if r % 11 == 0 {
				tilemap[x][y] = Tile::IRON;
			}
		}
	}

	tilemap
}

impl World {
	pub fn render_tilemap(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				let posf = Vector2f::new(x as f32, y as f32);

				let mut shape = RectangleShape::new();
				shape.set_fill_color(&self.tilemap[x][y].get_color());
				shape.set_position((posf - view.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
				shape.set_size(TILESIZE_VEC());
				window.draw(&shape);
			}
		}
	}

	pub fn get_tile(&self, p: Vector2u) -> &Tile {
		&self.tilemap[p.x as usize][p.y as usize]
	}
}
