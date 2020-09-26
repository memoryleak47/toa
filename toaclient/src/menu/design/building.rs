use sfml::graphics::Color;
use sfml::window::mouse::Button;

use toalib::world::Unit;
use toalib::command::{UnitCommand, Command};
use toalib::item::{Item, Inventory};
use toalib::vec::{Direction, Pos, Vec2f};

use crate::gameobject::GameObject;
use crate::graphics::{RawTextureId, TextureId};
use crate::app::App;
use crate::menu::{Widget, MenuState, MenuCommand};


impl App {
	pub(super) fn build_building_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let cursor = self.cursor;

		let b = if let Some(b) = self.world.buildingmap.get(cursor) { b }
		else { return Vec::new(); };

		let mut widgets = Vec::new();
		let ws = self.window_size();

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

		widgets
	}
}

