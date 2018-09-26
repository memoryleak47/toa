mod action_info;

use std::mem;

use toalib::team::PlayerID;
use toalib::world::World;
use toalib::world::aim::Aim;
use toalib::command::{Command, UnitCommand};
use toalib::misc::{Direction, vector_if, vector_iu, vector_ui};
use toalib::vec::{Vec2f, Vec2u};

use crate::input::Input;

#[derive(Debug, Copy, Clone)]
pub enum ItemUnitMode {
	Drop, Take, ChangeMainItem, Exec
}

pub enum UnitMode {
	Normal,
	Attack { aim: Aim },
	Build,
	Item { iu_mode: ItemUnitMode, index: usize },
	Craft { index: usize },
}

pub enum Action {
	ModeChange(Option<UnitMode>),
	MoveUnit(Direction),
	Command(Command),
	MoveCamera(Direction),
	MoveCursor(Direction),
	MoveAim(Direction),
	NextUnit,
}

struct Pending {
	time: u32,
	action: Action,
}

pub struct Controller {
	pub player_id: PlayerID,
	pub unit_mode: Option<UnitMode>, // None -> no unit focused
	pub focus_position: Vec2f,
	pub cursor: Vec2u,
	pending: Option<Pending>,
}

impl Controller {
	pub fn new(player_id: PlayerID) -> Controller {
		Controller {
			player_id,
			unit_mode: None,
			focus_position: Vec2f::new(0., 0.),
			cursor: Vec2u::new(0, 0),
			pending: None,
		}
	}

	pub fn command_accepted(&mut self, world: &World) {
		let mut pending = None;
		mem::swap(&mut pending, &mut self.pending);

		if let Some(x) = pending {
			x.action.execute(self, world);
		}
	}

	pub fn get_text(&self, world: &World) -> String {
		let pos = self.cursor;
		let terrain = world.get_terrain(pos);
		let building = world.get_building(pos);
		let unit = world.get_unit(pos).map(|x| x.get_info_string()).unwrap_or_else(|| "None".to_string());
		let inventory = world.get_inventory(pos);

		let default = format!("Terrain: {:?}\nBuilding: {}\nUnit: {}\nItems: {}", terrain, building.map(|x| x.get_class().get_name()).unwrap_or("None"), unit, inventory.get_info_string());
		let action_infos = self.get_action_infos(world);

		let v: Vec<_> = action_infos.iter()
				.map(|x| x.get_text())
				.collect();
		format!("{}\n{}", default, v.join("\n"))
	}

	pub fn tick(&mut self, w: &World, input: &Input) -> Option<Command> {
		if let Some(x) = self.pending.as_mut() {
			x.increase_time();
		} else {
			// in case the cursored unit died
			if w.get_unit(self.cursor)
					.filter(|x| x.owner == self.player_id)
					.is_none() {
				self.unit_mode = None;
			}

			let action_infos = self.get_action_infos(w);

			for info in action_infos.into_iter() {
				if info.is_triggered(input) {
					if let Some(x) = info.action.get_command(self.cursor) {
						if w.is_valid_command(self.player_id, &x) {
							self.pending = Some(Pending::new(info.action));
							return Some(x);
						}
					} else {
						info.action.execute(self, w);
					}
				}
			}
		}
		None
	}
}

impl Action {
	// this command has to be accepted by the server before the Action can be executed
	pub fn get_command(&self, pos: Vec2u) -> Option<Command> {
		match self {
			Action::Command(c) => Some(c.clone()),
			Action::MoveUnit(d) => Some(Command::UnitCommand { command: UnitCommand::Move(*d), pos }),
			_ => None,
		}
	}

	pub fn execute(self, controller: &mut Controller, w: &World) {
		match self {
			Action::NextUnit => {
				for x in w.find_next_unit_tile(controller.cursor, controller.player_id) {
					controller.cursor = x;
				}
			},
			Action::MoveUnit(d) => {
				controller.cursor = vector_iu(vector_ui(controller.cursor) + d.to_vector());
			},
			Action::ModeChange(m) => { controller.unit_mode = m; },
			Action::MoveAim(d) => {
				if let Some(UnitMode::Attack { ref mut aim }) = controller.unit_mode.as_mut() {
					aim.apply_direction(d, w);
				} else { assert!(false); }
			},
			Action::MoveCamera(d) => { controller.focus_position = vector_if(d.to_vector()) / 2. + controller.focus_position; },
			Action::MoveCursor(d) => { controller.cursor = d.plus_vector(controller.cursor); },
			Action::Command(_) => {},
		}
	}
}

impl Pending {
	fn new(action: Action) -> Pending {
		Pending {
			time: 0,
			action,
		}
	}

	fn increase_time(&mut self) {
		self.time += 1;
	}
}
