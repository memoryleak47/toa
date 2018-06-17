use sfml::window::Key;
use sfml::system::{Vector2f, Vector2u};

use player::Player;
use view::{View, Marker, MarkerType, CURSOR_COLOR};
use input::Input;
use world::World;
use command::Command;
use misc::{Direction, vector_if};

#[derive(Debug)]
enum UnitMode {
	Normal,
	Attack,
	Build
}

enum Action {
	ModeChange(Option<UnitMode>),
	Command(Command),
	MoveCamera(Direction),
	MoveCursor(Direction),
}

struct ActionInfo {
	text: String,
	action: Action,
	key_combination: Vec<Key>,
	fresh: bool, // whether it requires fresh keys
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
		vec![Marker {
			position: self.cursor,
			marker_type: MarkerType::Border,
			color: &CURSOR_COLOR,
		}]
	}

	fn get_action_infos(&self, w: &World) -> Vec<ActionInfo> {
		let mut v = Vec::new();

		v.push(ActionInfo {
			text: "next turn".to_string(),
			action: Action::Command(Command::NextTurn),
			key_combination: vec![Key::N],
			fresh: true,
		});

		// move camera:
		v.push(ActionInfo {
			text: "move camera up".to_string(),
			action: Action::MoveCamera(Direction::Up),
			key_combination: vec![Key::LControl, Key::W],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera left".to_string(),
			action: Action::MoveCamera(Direction::Left),
			key_combination: vec![Key::LControl, Key::A],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera down".to_string(),
			action: Action::MoveCamera(Direction::Down),
			key_combination: vec![Key::LControl, Key::S],
			fresh: false,
		});
		v.push(ActionInfo {
			text: "move camera right".to_string(),
			action: Action::MoveCamera(Direction::Right),
			key_combination: vec![Key::LControl, Key::D],
			fresh: false,
		});

		match self.unit_mode {
			Some(UnitMode::Normal) => {
				v.push(ActionInfo {
					text: "unfocus unit".to_string(),
					action: Action::ModeChange(None),
					key_combination: vec![Key::Escape],
					fresh: true,
				});
				v.push(ActionInfo {
					text: "move up".to_string(),
					action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Up}),
					key_combination: vec![Key::W],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move left".to_string(),
					action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Left}),
					key_combination: vec![Key::A],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move down".to_string(),
					action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Down}),
					key_combination: vec![Key::S],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move right".to_string(),
					action: Action::Command(Command::Move { from: self.cursor, direction: Direction::Right}),
					key_combination: vec![Key::D],
					fresh: false,
				});

				v.push(ActionInfo {
					text: "go to attack mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Attack)),
					key_combination: vec![Key::F],
					fresh: false,
				});

				v.push(ActionInfo {
					text: "go to build mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Build)),
					key_combination: vec![Key::B],
					fresh: false,
				});
			},
			Some(UnitMode::Attack) => {
				v.push(ActionInfo {
					text: "go to normal mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Normal)),
					key_combination: vec![Key::Escape],
					fresh: false,
				});
			}
			Some(UnitMode::Build) => {
				v.push(ActionInfo {
					text: "go to normal mode".to_string(),
					action: Action::ModeChange(Some(UnitMode::Normal)),
					key_combination: vec![Key::Escape],
					fresh: false,
				});
			}
			None => {
				v.push(ActionInfo {
					text: "move cursor up".to_string(),
					action: Action::MoveCursor(Direction::Up),
					key_combination: vec![Key::W],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move cursor left".to_string(),
					action: Action::MoveCursor(Direction::Left),
					key_combination: vec![Key::A],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move cursor down".to_string(),
					action: Action::MoveCursor(Direction::Down),
					key_combination: vec![Key::S],
					fresh: false,
				});
				v.push(ActionInfo {
					text: "move cursor right".to_string(),
					action: Action::MoveCursor(Direction::Right),
					key_combination: vec![Key::D],
					fresh: false,
				});
				if w.get_unit(self.cursor)
						.filter(|x| x.owner == self.player_id)
						.is_some() {
					v.push(ActionInfo {
						text: "focus unit".to_string(),
						action: Action::ModeChange(Some(UnitMode::Normal)),
						key_combination: vec![Key::Return],
						fresh: false,
					});
				}
			}
		}

		v = v.into_iter()
			.filter(|x| x.is_valid(self.player_id, w))
			.collect();

		v
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
				if let Some(x) = info.execute(self) {
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

impl ActionInfo {
	fn is_valid(&self, player_id: u32, w: &World) -> bool {
		if let Action::Command(ref c) = self.action {
			w.is_valid_command(player_id, c)
		} else {
			true
		}
	}

	fn get_text(&self) -> String {
		let v: Vec<_> = self.key_combination.iter()
			.map(|x| format!("{:?}", x))
			.collect();
		let key_string = v.join("+");
		format!("[{}]: {}", key_string, self.text)
	}

	fn is_triggered(&self, input: &Input) -> bool {
		if self.fresh {
			self.key_combination.iter()
				.all(|x| input.is_fresh_pressed(*x))
		} else {
			input.are_pressed_mod(&self.key_combination[..], 3)
		}
	}

	fn execute(self, player: &mut LocalPlayer) -> Option<Command> {
		match self.action {
			Action::Command(c) => return Some(c),
			Action::ModeChange(m) => { player.unit_mode = m; },
			Action::MoveCamera(d) => { player.focus_position = vector_if(d.to_vector()) / 2. + player.focus_position; },
			Action::MoveCursor(d) => { player.cursor = d.plus_vector(player.cursor); },
		}
		None
	}
}
