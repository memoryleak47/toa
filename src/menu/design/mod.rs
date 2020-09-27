mod item;
mod unit;
mod building;
mod terrain;

use crate::*;

impl App {
	pub fn on_tile_click(&self, p: Pos, b: Button) -> Vec<MenuCommand> {
		if let Button::Left = b {
			return match self.menu_state {
				MenuState::Attack(idx) => {
					let rel_mouse = self.get_world_mouse() - self.cursor.to_f();
					let cmd = UnitCommand::Attack(idx, rel_mouse);
					vec![
						MenuCommand::Command(Command::UnitCommand { command: cmd, pos: self.cursor })
					]
				},
				MenuState::DropChooseDir(ref indices) => {
					let v = vec![Some(Direction::Left), Some(Direction::Right), Some(Direction::Up), Some(Direction::Down), None];
					let mouse = if let Some(x) = self.get_world_mouse().to_i().to_pos() { x }
					else { return vec![]; };

					let local_pos = |opt_d: &Option<Direction>| -> Vec2i { opt_d.map(|d| *d).unwrap_or(Vec2i::new(0,0)) };
					let cond = |opt_d: &Option<Direction>| Some(mouse) == self.cursor.map(|x| x + local_pos(opt_d));
					if let Some(opt_dir) = v.into_iter().find(cond) {
						let cmd = |idx| Command::UnitCommand { command: UnitCommand::DropItem(idx, opt_dir), pos: self.cursor };

						let mut indices: Vec<_> = indices.iter().copied().collect();
						indices.sort();
						indices.into_iter().rev()
							.map(|idx| MenuCommand::Command(cmd(idx)))
							.chain(
								iter::once( MenuCommand::StateChange(MenuState::Normal))
							).collect()
					} else {
						vec![]
					}
				},
				_ => vec![MenuCommand::Cursor(p)],
			};
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
		let ws = self.window_size();
		let mut widgets = Vec::new();

		// left-side pane
		widgets.push(
			Widget {
				pos: (0.).into(),
				size: ws * (0.3, 1.),
				draw_type: self.pane_color().into(),
				on_click: vec![],
			},
		);

		widgets.extend(self.build_item_pane((0.0, 0.0).into()));
		widgets.extend(self.build_unit_pane((0.0, 0.25).into()));
		widgets.extend(self.build_building_pane((0.0, 0.5).into()));
		widgets.extend(self.build_terrain_pane((0.0, 0.75).into()));
		widgets.extend(self.main_button());

		widgets
	}

	pub fn main_button_cmds(&self) -> Vec<MenuCommand> {
		if let Some(p) = self.world.find_next_usable_unit_tile(self.cursor, self.player_id) {
			vec![MenuCommand::Cursor(p)]
		} else {
			if self.selected_unit().map(|u| u.stamina <= 0).unwrap_or(false) {
				vec![MenuCommand::Command(Command::NextTurn)]
			} else {
				vec![]
			}
		}
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
				on_click: self.main_button_cmds(),
			},
		);

		widgets
	}

	fn selected_unit(&self) -> Option<&'_ Unit> {
		self.world.unitmap.get(self.cursor)
	}

	fn pane_color(&self) -> Color {
		if let Some(unit) = self.selected_unit() {
			let red_add = (unit.food < 20) as u8 * 70;
			if unit.stamina <= 0 { Color::rgb(70 + red_add, 70, 70) }
			else { Color::rgb(100 + red_add, 100, 100) }
		} else {
			Color::rgb(100, 100, 100)
		}
	}
}
