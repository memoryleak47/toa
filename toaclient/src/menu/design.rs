use sfml::graphics::Color;

use toalib::world::Unit;

use crate::app::App;
use crate::menu::{Widget, MenuCommand};

impl App {
	pub fn generate_widgets(&self) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		widgets.extend(self.build_pane());
		widgets.extend(self.main_button());

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


		widgets.push(Widget {
			pos: ws * (0.01),
			size: ws * (0.25, 0.25),
			draw_type: format!("health: {}", u.health).into(),
			on_click: None,
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.03),
			size: ws * (0.25, 0.25),
			draw_type: format!("food: {}", u.food).into(),
			on_click: None,
		});

		widgets.push(Widget {
			pos: ws * (0.01, 0.05),
			size: ws * (0.25, 0.25),
			draw_type: format!("stamina: {}", u.stamina).into(),
			on_click: None,
		});

		widgets
	}

	fn main_button(&self) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		widgets.push(
			Widget {
				pos: ws * 0.95,
				size: ws * 0.03,
				draw_type: Color::rgb(100, 100, 100).into(),
				on_click: Some(MenuCommand::NextTurn),
			},
		);

		widgets
	}
}
