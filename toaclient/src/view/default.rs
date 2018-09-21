use sfml::system::{Vector2f, Vector2u};
use sfml::window::Key;

use crate::input::Input;
use crate::view::{View, Marker, MarkerType, CURSOR_COLOR};
use crate::world::World;

pub struct DefaultViewGenerator {
	focus_position: Vector2f,
	cursor: Vector2u,
	player: u32,
}

impl DefaultViewGenerator {
	pub fn new(player: u32) -> DefaultViewGenerator {
		DefaultViewGenerator {
			focus_position: Vector2f::new(0., 0.),
			cursor: Vector2u::new(0, 0),
			player
		}
	}

	// This implements the view-functionality, which is used if the active-player is no LocalPlayer, it should be called in *Player::tick()
	pub fn tick(&mut self, input: &Input) {
		if let Some(direction) = input.move_direction() {
			if input.is_pressed(Key::LControl) || input.is_pressed(Key::RControl) {
				let v = direction.to_vector();
				self.focus_position += Vector2f::new(v.x as f32, v.y as f32) / 2.;
			} else {
				self.cursor = direction.plus_vector(self.cursor);
			}
		}
	}

	// This implements the view-functionality, which is used if the active-player is no LocalPlayer, it should be called in *Player::get_view()
	pub fn get_view(&self, w: &World) -> View {
		View {
			markers: self.get_markers(),
			focus_position: self.focus_position,
			text: View::default_text_at(self.cursor, w),
			player: self.player,
		}
	}

	fn get_markers(&self) -> Vec<Marker> {
		vec![Marker {
			position: self.cursor,
			marker_type: MarkerType::Border,
			color: &CURSOR_COLOR,
		}]
	}

}

impl View {
	pub fn default_text_at(pos: Vector2u, world: &World) -> String {
		let terrain = world.get_terrain(pos);
		let building = world.get_building(pos);
		let unit = world.get_unit(pos).map(|x| x.get_info_string()).unwrap_or_else(|| "None".to_string());
		let inventory = world.get_inventory(pos);

		format!("Active Player: {:?}\nTerrain: {:?}\nBuilding: {}\nUnit: {}\nItems: {}", world.active_player, terrain, building.map(|x| x.get_class().get_name()).unwrap_or("None"), unit, inventory.get_info_string())
	}
}
