use crate::*;

impl App {
	pub(super) fn build_terrain_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let t = self.world.terrainmap.get(self.cursor);
		let ws = self.window_size();
		let mut widgets = Vec::new();
		let cursor = self.cursor;

		widgets.push(Widget {
			pos: ws * (offset + 0.01),
			size: ws * (0.025, 0.025),
			draw_type: format!("terrain: {}", t.str()).into(),
			on_click: vec![],
			hotkey: None,
		});

		let cmd = Command::UnitCommand{ command: UnitCommand::TerrainWork, pos: cursor };

		// TODO check whether its a valid command
		widgets.push(Widget {
			pos: ws * (offset + (0.01, 0.04)),
			size: ws * 0.025,
			draw_type: Color::rgb(10, 10, 10).into(),
			on_click: vec![MenuCommand::Command(cmd)],
			hotkey: Some(TERRAIN_WORK_HOTKEY),
		});

		widgets
	}
}

