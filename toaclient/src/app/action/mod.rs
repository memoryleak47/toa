mod info;

use toalib::team::PlayerID;
use toalib::world::World;
use toalib::command::{Command, UnitCommand};
use toalib::vec::Direction;
use toalib::vec::Pos;

use crate::unit_mode::UnitMode;
use crate::app::App;

pub enum Action {
	ModeChange(Option<UnitMode>),
	MoveUnit {
		pos: Pos,
		direction: Direction,
	},
	RawCommand(Command),
	BackCommand(Command), // a command after which unit_mode should be back to normal mode
	MoveCamera(Direction),
	MoveCursor(Direction),
	MoveAim(Direction),
	ZoomIn,
	ZoomOut,
	NextUnit,
}

impl App {
	// this functino assumes that the action is valid
	pub fn execute_action(&mut self, action: Action) {
		match action {
			Action::NextUnit => {
				for x in self.world.find_next_unit_tile(self.cursor, self.player_id) {
					self.cursor = x;
				}
				self.unit_mode = None;
			},
			Action::MoveUnit { direction, pos } => {
				self.cursor = pos.map(|x| x + *direction).unwrap();
			},
			Action::ModeChange(m) => { self.unit_mode = m; },
			Action::MoveAim(d) => {
				if let Some(UnitMode::Attack { ref mut aim }) = self.unit_mode.as_mut() {
					aim.apply_direction(d, &self.world);
				} else { assert!(false); }
			},
			Action::MoveCamera(d) => { self.focus_position = (*d).to_f() / 2. + self.focus_position; },
			Action::MoveCursor(d) => {
				if let Some(p) = self.cursor.map(|x| x + *d) {
					self.cursor = p;
				}
			},
			Action::ZoomIn => { self.tilesize *= 1.1; },
			Action::ZoomOut => { if self.tilesize > 0. { self.tilesize /= 1.1; } },
			Action::RawCommand(_) => {},
			Action::BackCommand(_) => {
				self.unit_mode = Some(UnitMode::Normal);
			}
		}
	}

}

impl Action {
	// this command has to be accepted by the server before the Action can be executed
	pub fn get_command(&self) -> Option<Command> {
		match self {
			Action::RawCommand(c) => Some(c.clone()),
			Action::BackCommand(c) => Some(c.clone()),
			Action::MoveUnit { direction, pos } => Some(Command::UnitCommand { command: UnitCommand::Move(*direction), pos: *pos }),
			_ => None,
		}
	}

	pub fn is_valid(&self, w: &World, player_id: PlayerID) -> bool {
		self.get_command()
			.map(|c| w.is_valid_command(player_id, &c))
			.unwrap_or(true)
	}
}
