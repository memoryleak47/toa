mod action_info;

use self::action_info::{Action, ActionInfo};

use sfml::window::Key;
use sfml::system::{Vector2f, Vector2u};

use player::Player;
use view::{View, Marker, MarkerType, CURSOR_COLOR, TARGET_CURSOR_COLOR};
use input::Input;
use world::World;
use command::Command;
use misc::{Direction, vector_if};

#[derive(Debug)]
pub enum UnitMode {
	Normal,
	Attack { target_cursor: Vector2u },
	Build
}

pub struct LocalPlayer {
	player_id: u32,
	unit_mode: Option<UnitMode>, // None -> no unit focused
	focus_position: Vector2f,
	cursor: Vector2u,
}

impl LocalPlayer {
	pub fn new(player_id: u32) -> LocalPlayer {
		LocalPlayer {
			player_id,
			unit_mode: None,
			focus_position: Vector2f::new(0., 0.),
			cursor: Vector2u::new(0, 0),
		}
	}

	fn get_text(&self, w: &World) -> String {
		let default = View::default_text_at(self.cursor, w);
		let action_infos = self.get_action_infos(w);

		let v: Vec<_> = action_infos.iter()
				.map(|x| x.get_text())
				.collect();
		format!("{}\n\nMode: {:?}\n{}", default, self.unit_mode, v.join("\n"))
	}

	fn get_markers(&self) -> Vec<Marker> {
		let mut v = Vec::new();

		v.push(Marker {
			position: self.cursor,
			marker_type: MarkerType::Border,
			color: &CURSOR_COLOR,
		});

		if let Some(UnitMode::Attack { target_cursor }) = self.unit_mode {
			v.push(Marker {
				position: target_cursor,
				marker_type: MarkerType::Border,
				color: &TARGET_CURSOR_COLOR,
			});
		}

		v
	}

	fn apply_view_command(&mut self, command: &Command) {
		match command {
			Command::Move { from, direction } => {
				self.cursor = direction.plus_vector(self.cursor);
			},
			_ => {}
		}
	}
}

impl Player for LocalPlayer {
	fn tick(&mut self, w: &World, input: &Input) -> Option<Command> {
		// in case the cursored unit died
		if w.get_unit(self.cursor)
				.filter(|x| x.owner == self.player_id)
				.is_none() {
			self.unit_mode = None;
		}

		let action_infos = self.get_action_infos(w);

		for info in action_infos.into_iter() {
			if info.is_triggered(input) {
				if let Some(x) = info.execute(self, w) {
					self.apply_view_command(&x);
					return Some(x);
				}
			}
		}
		None
	}

	fn get_view(&self, w: &World) -> View {
		View {
			markers: self.get_markers(),
			focus_position: self.focus_position,
			player: self.player_id,
			text: self.get_text(w),
		}
	}

	fn turn_start(&mut self) {
		self.unit_mode = None;
	}
}

