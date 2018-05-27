use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::system::Vector2f;

use view::View;

use world::{TILESIZE, TILESIZE_VEC, MAP_SIZE};

#[derive(Copy, Clone)]
enum BuildingKind {
	Spawn,
}

#[derive(Copy, Clone)]
pub struct Building {
	owner: u8,
	kind: BuildingKind,
}

impl Building {
	fn get_color(&self) -> Color {
		if self.owner == 0 {
			Color::rgba(255, 0, 0, 100)
		} else {
			Color::rgba(0, 0, 255, 100)
		}
	}
}

pub struct BuildingMap {
	buildings: [[Option<Building>; MAP_SIZE]; MAP_SIZE],
}

impl BuildingMap {
	pub fn gen() -> BuildingMap {
		let mut buildings = [[None; MAP_SIZE]; MAP_SIZE];

		buildings[MAP_SIZE / 2][0] = Some(Building { owner: 0, kind: BuildingKind::Spawn });
		buildings[MAP_SIZE / 2][MAP_SIZE - 1] = Some(Building { owner: 1, kind: BuildingKind::Spawn });

		BuildingMap { buildings }
	}

	pub fn render(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if let Some(building) = self.buildings[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);
					let size = window.size();

					let mut shape = RectangleShape::new();
					shape.set_fill_color(&building.get_color());
					shape.set_position((posf - view.focus_position) * TILESIZE + Vector2f::new(size.x as f32, size.y as f32) / 2.0);
					shape.set_size(TILESIZE_VEC());
					window.draw(&shape);
				}
			}
		}
	}
}