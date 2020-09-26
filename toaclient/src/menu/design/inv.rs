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
	pub(super) fn build_inventory_pane(&self, offset: Vec2f, inv: &Inventory) -> Vec<Widget> {
		let mut widgets = vec![];
		let ws = self.window_size();

		for (i, item) in inv.iter().enumerate() {
			widgets.push(
				Widget {
					pos: ws * (offset + (0.03 * i as f32, 0.0)),
					size: ws * (0.025, 0.025),
					draw_type: item.get_texture_id().into(),
					on_click: vec![],
				},
			);
		}

		widgets
	}
}