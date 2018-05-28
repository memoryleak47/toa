use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};
use rand::{RngCore, thread_rng};

use view::View;
use world::{World, TILESIZE, TILESIZE_VEC, MAP_SIZE_X, MAP_SIZE_Y};
use misc::vector_uf;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Terrain {
	GRASS,
	FOREST,
	STONE,
	IRON,
}

impl Terrain {
	pub fn get_color(&self) -> Color {
		match self {
			Terrain::GRASS => Color::rgb(50,150,50),
			Terrain::FOREST => Color::rgb(0,50,0),
			Terrain::STONE => Color::rgb(50,50,50),
			Terrain::IRON => Color::rgb(150,150,150),
		}
	}

	pub fn get_stamina_cost(&self) -> u32 {
		match self {
			Terrain::GRASS => 10,
			Terrain::FOREST => 20,
			Terrain::STONE => 20,
			Terrain::IRON => 20,
		}
	}
}

pub fn new_terrainmap() -> [[Terrain; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut rng = thread_rng();

	let mut terrainmap = [[Terrain::GRASS; MAP_SIZE_Y]; MAP_SIZE_X];
	for x in 0..MAP_SIZE_X {
		for y in 0..MAP_SIZE_Y {
			let r = rng.next_u32();
			if r % 3 == 0 {
				terrainmap[x][y] = Terrain::FOREST;
			} else if r % 7 == 0 {
				terrainmap[x][y] = Terrain::STONE;
			} else if r % 11 == 0 {
				terrainmap[x][y] = Terrain::IRON;
			}
		}
	}

	terrainmap
}

impl World {
	pub fn render_terrainmap(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let posf = Vector2f::new(x as f32, y as f32);

				let mut shape = RectangleShape::new();
				shape.set_fill_color(&self.terrainmap[x][y].get_color());
				shape.set_position((posf - view.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
				shape.set_size(TILESIZE_VEC());
				window.draw(&shape);
			}
		}
	}

	pub fn get_terrain(&self, p: Vector2u) -> &Terrain {
		&self.terrainmap[p.x as usize][p.y as usize]
	}
}
