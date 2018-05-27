use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Shape, Color, Transformable};
use sfml::system::Vector2f;

use view::View;

use world::{TILESIZE, MAP_SIZE};

#[derive(Copy, Clone)]
pub struct Unit {
	owner: u8,
}

impl Unit {
	fn get_color(&self) -> Color {
		if self.owner == 0 {
			Color::rgb(255, 0, 0)
		} else {
			Color::rgb(0, 0, 255)
		}
	}
}

pub struct UnitMap {
	units: [[Option<Unit>; MAP_SIZE]; MAP_SIZE],
}

impl UnitMap {
	pub fn gen() -> UnitMap {
		let mut units = [[None; MAP_SIZE]; MAP_SIZE];

		units[MAP_SIZE / 2][0] = Some(Unit { owner: 0 });
		units[MAP_SIZE / 2][MAP_SIZE - 1] = Some(Unit { owner: 1 });

		UnitMap { units }
	}

	pub fn render(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if let Some(unit) = self.units[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);
					let size = window.size();

					let mut shape = CircleShape::new(TILESIZE / 2.0, 200);
					shape.set_fill_color(&unit.get_color());
					shape.set_position((posf - view.focus_position) * TILESIZE + Vector2f::new(size.x as f32, size.y as f32) / 2.0);
					window.draw(&shape);
				}
			}
		}
	}
}
