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
	pub(super) fn build_item_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let mut widgets = Vec::new();

		widgets
	}
}

