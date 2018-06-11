use sfml::window::Key;
use sfml::system::Vector2f;

use player::Player;
use view::View;
use input::Input;
use world::{buildingmap::BUILDING_PLANS, World};
use command::Command;

pub struct LocalPlayer {
	marking_unit: bool,
}

impl LocalPlayer {
	pub fn new() -> LocalPlayer {
		LocalPlayer { marking_unit: false }
	}
}

impl Player for LocalPlayer {
	fn tick(&mut self, w: &World, view: &mut View, input: &Input) -> Option<Command> {
		if input.is_pressed(Key::Escape) {
			self.marking_unit = false;
		}

		// move main_cursor
		if let Some(direction) = input.move_direction() {
			if input.is_pressed(Key::LControl) || input.is_pressed(Key::RControl) {
				let v = direction.to_vector();
				view.focus_position += Vector2f::new(v.x as f32, v.y as f32);
			} else if !self.marking_unit {
				view.move_cursor(direction);
			}
		}

		if input.is_fresh_pressed(Key::Return) && !self.marking_unit {
			if let Some(unit) = w.get_unit(view.main_cursor) {
				if unit.owner == w.active_player {
					self.marking_unit = true;
				}
			}
		}

		if input.is_fresh_pressed(Key::N) {
			return Some(Command::NextTurn);
		}

		if self.marking_unit {
			if let Some(direction) = input.move_direction() {
				return Some(Command::Move { from: view.main_cursor, direction });
			}

			if input.is_fresh_pressed(Key::F) {
				return Some(Command::Build { at: view.main_cursor, plan: &BUILDING_PLANS[0]})
			}

			if input.is_fresh_pressed(Key::J) {
				return Some(Command::Work { at: view.main_cursor})
			}
		} else if input.is_fresh_pressed(Key::U) {
			view.main_cursor = w.find_next_unit_tile(view.main_cursor, w.active_player).unwrap();
		}

		None
	}

	fn turn_start(&mut self) {
		self.marking_unit = false;
	}

	fn uses_view(&self) -> bool { true }
}
