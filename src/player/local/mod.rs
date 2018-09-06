mod action_info;

use sfml::system::{Vector2f, Vector2u};

use player::Player;
use view::{View, Marker, MarkerType, CURSOR_COLOR, TARGET_CURSOR_COLOR};
use input::Input;
use world::World;
use world::aim::Aim;
use command::{Command, UnitCommand};
use misc::{Direction, vector_if, vector_iu, vector_ui};

#[derive(Debug, Copy, Clone)]
pub enum ItemUnitMode {
	Drop, Take, ChangeMainItem
}

pub enum UnitMode {
	Normal,
	Attack { aim: Box<dyn Aim> },
	Build,
	Item { iu_mode: ItemUnitMode, index: usize },
	Craft { index: usize },
}

pub enum Action {
	ModeChange(Option<UnitMode>),
	Command(Command),
	MoveCamera(Direction),
	MoveCursor(Direction),
	MoveAim(Direction),
	NextUnit,
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
		format!("{}\n{}", default, v.join("\n"))
	}

	fn get_markers(&self) -> Vec<Marker> {
		let mut v = Vec::new();

		v.push(Marker {
			position: self.cursor,
			marker_type: MarkerType::Border,
			color: &CURSOR_COLOR,
		});

		if let Some(UnitMode::Attack { ref aim }) = self.unit_mode {
			v.extend(
				aim.get_relative_tiles()
					.iter()
					.map(|x| *x + vector_ui(self.cursor))
					.filter(|x| x.x >= 0 && x.y >= 0)
					.map(|x| vector_iu(x))
					.map(|x| Marker {
						position: x,
						marker_type: MarkerType::Transparent,
						color: &TARGET_CURSOR_COLOR,
					})
			);
		}
		v
	}

	fn apply_view_command(&mut self, command: &Command) {
		match command {
			Command::UnitCommand { command: UnitCommand::Move(direction), .. } => {
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
				if let Some(x) = info.action.execute(self, w) {
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

impl Action {
	pub fn execute(self, player: &mut LocalPlayer, w: &World) -> Option<Command> {
		match self {
			Action::Command(c) => return Some(c),
			Action::NextUnit => {
				for x in w.find_next_unit_tile(player.cursor, player.player_id) {
					player.cursor = x;
				}
			}
			Action::ModeChange(m) => { player.unit_mode = m; },
			Action::MoveAim(d) => {
				if let Some(UnitMode::Attack { ref mut aim }) = player.unit_mode.as_mut() {
					aim.apply_direction(d, w);
				} else { assert!(false); }
			},
			Action::MoveCamera(d) => { player.focus_position = vector_if(d.to_vector()) / 2. + player.focus_position; },
			Action::MoveCursor(d) => { player.cursor = d.plus_vector(player.cursor); },
		}
		None
	}
}
