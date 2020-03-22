use sfml::graphics::Color;

use toalib::world::Unit;
use toalib::command::{UnitCommand, Command};
use toalib::item::{Item, Inventory};

use crate::app::App;
use crate::menu::{Widget, MenuState, ItemChoiceMode};

impl App {
	pub fn generate_widgets(&self) -> Vec<Widget> {
		let mut widgets = Vec::new();

		widgets.extend(self.build_pane());
		widgets.extend(self.main_button());

		match self.menu_state {
			MenuState::Normal => {},
			MenuState::ItemChoice(ItemChoiceMode::Attack) => {
				widgets.extend(self.build_attack_mode());
			},
			MenuState::Attack => {}, // TODO
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
				on_click: None,
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
			on_click: None,
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.03),
			size: ws * (0.025, 0.025),
			draw_type: format!("food: {}", u.food).into(),
			on_click: None,
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.05),
			size: ws * (0.025, 0.025),
			draw_type: format!("stamina: {}", u.stamina).into(),
			on_click: None,
		});
		widgets.push(Widget {
			pos: ws * (0.01, 0.08),
			size: ws * 0.025,
			draw_type: Color::rgb(30, 30, 30).into(),
			on_click: Some(Box::new(move |a| a.send_command(Command::UnitCommand{ command: UnitCommand::Work, pos: cursor }, None))),
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.11),
			size: ws * 0.025,
			draw_type: Color::rgb(30, 30, 30).into(),
			on_click: Some(Box::new(move |a| a.send_command(Command::UnitCommand{ command: UnitCommand::UnrefinedWork, pos: cursor }, None))),
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.14),
			size: ws * 0.025,
			draw_type: Color::rgb(100, 0, 0).into(),
			on_click: Some(Box::new(move |a| { a.menu_state = MenuState::ItemChoice(ItemChoiceMode::Attack); }))
		});

		widgets
	}

	fn build_attack_mode(&self) -> Vec<Widget> {
		let mut inv = self.world.unitmap.get(self.cursor).unwrap().inventory.clone();
		// TODO inv.push(Hand);
		self.build_inventory(inv, Box::new(|a, _| { a.menu_state = MenuState::Attack; }))
	}

	fn build_inventory(&self, inv: Inventory, reaction: Box<dyn Fn(&mut App, Item)>) -> Vec<Widget> {
		vec![] // TODO
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
				on_click: Some(Box::new(|a| { a.send_command(Command::NextTurn, None); } )),
			},
		);

		widgets
	}
}
