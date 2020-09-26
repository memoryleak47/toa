use sfml::graphics::Color;
use sfml::window::mouse::Button;

use toalib::world::{Unit, BUILDABLE_BUILDING_CLASSES, BuildingClass};
use toalib::command::{UnitCommand, Command};
use toalib::item::{Item, Inventory};
use toalib::vec::{Direction, Pos, Vec2f};

use crate::gameobject::{GameObject, building::BuildingClassGetTextureId};
use crate::graphics::{RawTextureId, TextureId};
use crate::app::App;
use crate::menu::{Widget, MenuState, MenuCommand};


impl App {
	pub(super) fn build_building_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let cursor = self.cursor;

		let opt_b =  self.world.buildingmap.get(cursor);

		let mut widgets = Vec::new();
		let ws = self.window_size();

		if let Some(b) = opt_b {
			widgets.push(Widget {
				pos: ws * (offset + (0.01, 0.05)),
				size: ws * (0.025, 0.025),
				draw_type: b.get_info_string().into(),
				on_click: vec![],
			});

			widgets.push(Widget {
				pos: ws * (offset + (0.01, 0.08)),
				size: ws * 0.025,
				draw_type: Color::rgb(30, 30, 30).into(),
				on_click: vec![MenuCommand::Command(Command::UnitCommand{ command: UnitCommand::Work, pos: cursor })],
			});

			widgets.push(Widget {
				pos: ws * (offset + (0.04, 0.08)),
				size: ws * 0.025,
				draw_type: Color::rgb(100, 30, 30).into(),
				on_click: vec![MenuCommand::Command(Command::UnitCommand{ command: UnitCommand::BurnBuilding, pos: cursor })],
			});
		} else {
			widgets.push(Widget {
				pos: ws * (offset + (0.01, 0.08)),
				size: ws * 0.025,
				draw_type: Color::rgb(70, 50, 0).into(),
				on_click: vec![MenuCommand::StateChange(MenuState::Build)],
			});
		}

		let offset = offset + (0.0, 0.11);
		match self.menu_state {
			MenuState::Build => widgets.extend(self.build_build_pane(offset)),
			_ => {},
		}

		widgets
	}

	// naming though
	fn build_build_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let mut widgets = Vec::new();
		let ws = self.window_size();

		let on_click = |c: BuildingClass| {
			let cmd = UnitCommand::Build(c);
			vec![
				MenuCommand::Command(Command::UnitCommand { command: cmd, pos: self.cursor }),
				MenuCommand::StateChange(MenuState::Normal),
			]
		};

		for (i, &c) in BUILDABLE_BUILDING_CLASSES.iter().enumerate() {
			widgets.push(
				Widget {
					pos: ws * (offset + (0.03 * i as f32 + 0.01, 0.0)),
					size: ws * (0.025, 0.025),
					draw_type: c.get_texture_id().into(),
					on_click: on_click(c),
				},
			);
		}

		widgets
	}
}

