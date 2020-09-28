use crate::*;

impl App {
	pub(super) fn build_item_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		widgets.push(Widget {
			pos: ws * (offset + (0.01, 0.01)),
			size: ws * 0.025,
			draw_type: if matches!(self.menu_state, MenuState::TakeItem) { Color::rgb(0, 0, 200) } else { Color::rgb(0, 0, 100) }.into(),
			on_click: vec![MenuCommand::StateChange(MenuState::TakeItem) ],
			hotkey: Some(Key::T),
		});

		let on_click = |i| {
			if matches!(self.menu_state, MenuState::TakeItem) {
				let cmd = UnitCommand::TakeItem(i);
				vec![
					MenuCommand::Command(Command::UnitCommand { command: cmd, pos: self.cursor }),
					MenuCommand::StateChange(MenuState::Normal),
				]
			} else {
				vec![]
			}
		};

		let inv = self.world.itemmap.get(self.cursor);
		for (i, item) in inv.iter().enumerate() {
			widgets.push(
				Widget {
					pos: ws * (offset + (0.03 * i as f32 + 0.01, 0.04)),
					size: ws * (0.025, 0.025),
					draw_type: item.get_texture_id().into(),
					on_click: on_click(i),
					hotkey: numeric_hotkey(i+1),
				},
			);
		}

		widgets
	}
}

