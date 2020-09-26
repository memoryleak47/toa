use sfml::graphics::Color;
use sfml::window::mouse::Button;

use toalib::world::Unit;
use toalib::command::{UnitCommand, Command};
use toalib::item::{Item, Inventory};
use toalib::vec::{Direction, Pos, Vec2f};

use crate::gameobject::GameObject;
use crate::graphics::{RawTextureId, TextureId};
use crate::app::App;
use crate::menu::{Widget, MenuState, MenuCommand, ItemChoiceMode};


impl App {
	pub(super) fn build_unit_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let u = if let Some(x) = self.world.unitmap.get(self.cursor) { x } else { return Vec::new(); };
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
			draw_type: Color::rgb(100, 0, 0).into(),
			on_click: vec![MenuCommand::StateChange(MenuState::Attack(None)) ],
		});

		widgets.extend(self.build_unit_inv_pane(u, (0.01, 0.14).into()));

		widgets
	}

	fn build_unit_inv_pane(&self, u: &Unit, offset: Vec2f) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		let attack_marked = |i| {
			if let MenuState::Attack(Some(j)) = self.menu_state {
				i == j
			} else { false }
		};

		let on_click = |i| match self.menu_state {
			MenuState::Attack(_) => {
				vec![MenuCommand::StateChange(MenuState::Attack(Some(i)))]
			},
			_ => Vec::new(),
		};

		for (i, item) in u.inventory.iter().enumerate() {
			if attack_marked(i) {
				widgets.push(
					Widget {
						pos: ws * (offset + (0.03 * i as f32, 0.0)),
						size: ws * (0.025, 0.025),
						draw_type: Color::rgba(255, 0, 0, 20).into(),
						on_click: vec![],
					});
			}

			widgets.push(
				Widget {
					pos: ws * (offset + (0.03 * i as f32, 0.0)),
					size: ws * (0.025, 0.025),
					draw_type: item.get_texture_id().into(),
					on_click: on_click(i),
				},
			);
		}

		widgets
	}
}
