use sfml::graphics::Color;
use sfml::window::mouse::Button;

use toalib::world::Unit;
use toalib::command::{UnitCommand, Command};
use toalib::item::{Item, Inventory};
use toalib::vec::{Direction, Pos};

use crate::gameobject::GameObject;
use crate::graphics::{RawTextureId, TextureId};
use crate::app::App;
use crate::menu::{Widget, MenuState, MenuCommand, ItemChoiceMode};

impl App {
	pub fn on_tile_click(&self, p: Pos, b: Button) -> Vec<MenuCommand> {
		if let Button::Left = b {
			return vec![MenuCommand::Cursor(p)];
		}
		if let Button::Right = b {
			if let Some(d) = [Direction::Left, Direction::Right, Direction::Up, Direction::Down].iter()
						.find(|&d| self.cursor.map(|x| x + **d) == Some(p)) {
				return vec![
					MenuCommand::Command(Command::UnitCommand { command: UnitCommand::Move(*d), pos: self.cursor }),
					MenuCommand::Cursor(p)
				];
			}
		}
		vec![]
	}

	pub fn generate_widgets(&self) -> Vec<Widget> {
		let mut widgets = Vec::new();

		widgets.extend(self.build_pane());
		widgets.extend(self.main_button());

		match self.menu_state {
			MenuState::Normal => {},
			MenuState::ItemChoice(ItemChoiceMode::Attack) => {
				widgets.extend(self.build_attack_choice_mode());
			},
			MenuState::Attack(_) => {}, // TODO
		}

		widgets
	}

	fn build_pane(&self) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		widgets.push(
			Widget {
				pos: (0.).into(),
				size: ws * (0.3, 1.),
				draw_type: Color::rgb(100, 100, 100).into(),
				on_click: vec![],
			},
		);

		if let Some(u) = self.world.unitmap.get(self.cursor) {
			widgets.extend(self.build_unit_pane(u));
		}

		widgets
	}

	fn build_unit_pane(&self, u: &Unit) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();
		let cursor = self.cursor;

		widgets.push(Widget {
			pos: ws * (0.01),
			size: ws * (0.025, 0.025),
			draw_type: format!("health: {}", u.health).into(),
			on_click: vec![],
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.03),
			size: ws * (0.025, 0.025),
			draw_type: format!("food: {}", u.food).into(),
			on_click: vec![],
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.05),
			size: ws * (0.025, 0.025),
			draw_type: format!("stamina: {}", u.stamina).into(),
			on_click: vec![],
		});
		widgets.push(Widget {
			pos: ws * (0.01, 0.08),
			size: ws * 0.025,
			draw_type: Color::rgb(30, 30, 30).into(),
			on_click: vec![MenuCommand::Command(Command::UnitCommand{ command: UnitCommand::Work, pos: cursor })],
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.11),
			size: ws * 0.025,
			draw_type: Color::rgb(30, 30, 30).into(),
			on_click: vec![MenuCommand::Command(Command::UnitCommand{ command: UnitCommand::UnrefinedWork, pos: cursor })],
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.14),
			size: ws * 0.025,
			draw_type: Color::rgb(100, 0, 0).into(),
			on_click: vec![MenuCommand::StateChange(MenuState::ItemChoice(ItemChoiceMode::Attack)) ],
		});

		widgets
	}

	fn build_attack_choice_mode(&self) -> Vec<Widget> {
		let inv = &self.world.unitmap.get(self.cursor).unwrap().inventory;

		let mut widgets = vec![];
		let ws = self.window_size();

		// pane
		widgets.push(
			Widget {
				pos: ws * (0.3, 0.0),
				size: ws * (0.3, 1.),
				draw_type: Color::rgb(100, 100, 100).into(),
				on_click: vec![],
			},
		);

		// hand:
		widgets.push(
			Widget {
				pos: ws * (0.3, 0.0),
				size: ws * (0.025, 0.025),
				draw_type: TextureId::from(RawTextureId::Hand).into(),
				on_click: vec![MenuCommand::StateChange(MenuState::Attack(None))],
			},
		);

		for (i, item) in inv.iter().enumerate() {
			widgets.push(
				Widget {
					pos: ws * (0.3 + 0.03 * (i+1) as f32, 0.0),
					size: ws * (0.025, 0.025),
					draw_type: item.get_texture_id().into(),
					on_click: vec![MenuCommand::StateChange(MenuState::Attack(Some(i)))],
				},
			);
		}

		widgets
	}

	fn main_button(&self) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		let s = (ws.x * 0.01).into();
		widgets.push(
			Widget {
				pos: ws - s,
				size: s,
				draw_type: Color::rgb(100, 100, 100).into(),
				on_click: vec![MenuCommand::Command(Command::NextTurn)],
			},
		);

		widgets
	}
}
