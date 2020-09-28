use crate::*;

impl App {
	pub(super) fn build_unit_pane(&self, offset: Vec2f) -> Vec<Widget> {
		let u = if let Some(x) = self.world.unitmap.get(self.cursor) { x } else { return Vec::new(); };
		let ws = self.window_size();
		let mut widgets = Vec::new();

		widgets.push(Widget {
			pos: ws * (offset + (0.01)),
			size: ws * (0.025, 0.025),
			draw_type: format!("health: {}", u.health).into(),
			on_click: vec![],
			hotkey: None,
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.01, 0.03)),
			size: ws * (0.025, 0.025),
			draw_type: format!("food: {}", u.food).into(),
			on_click: vec![],
			hotkey: None,
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.01, 0.05)),
			size: ws * (0.025, 0.025),
			draw_type: format!("stamina: {}", u.stamina).into(),
			on_click: vec![],
			hotkey: None,
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.01, 0.08)),
			size: ws * 0.025,
			draw_type: if matches!(self.menu_state, MenuState::Attack(_)) { Color::rgb(200, 0, 0) } else { Color::rgb(100, 0, 0) }.into(),
			on_click: vec![MenuCommand::StateChange(MenuState::Attack(None)) ],
			hotkey: Some(ATTACK_HOTKEY),
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.04, 0.08)),
			size: ws * 0.025,
			draw_type: if matches!(self.menu_state, MenuState::ExecItem) { Color::rgb(0, 200, 0) } else { Color::rgb(0, 100, 0) }.into(),
			on_click: vec![MenuCommand::StateChange(MenuState::ExecItem) ],
			hotkey: Some(EXEC_ITEM_HOTKEY)
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.07, 0.08)),
			size: ws * 0.025,
			draw_type: if matches!(self.menu_state, MenuState::DropChooseItem | MenuState::DropChooseDir(_)) { Color::rgb(0, 0, 200) } else { Color::rgb(0, 0, 100) }.into(),
			on_click: vec![MenuCommand::StateChange(MenuState::DropChooseItem)],
			hotkey: Some(DROP_ITEM_HOTKEY)
		});

		widgets.push(Widget {
			pos: ws * (offset + (0.10, 0.08)),
			size: ws * 0.025,
			draw_type: Color::rgb(130, 130, 130).into(),
			on_click: vec![MenuCommand::Command(Command::UnitCommand { command: UnitCommand::Idle, pos: self.cursor })],
			hotkey: Some(IDLE_HOTKEY),
		});

		widgets.extend(self.build_unit_inv_pane(u, offset + (0.00, 0.11)));

		widgets
	}

	fn build_unit_inv_pane(&self, u: &Unit, offset: Vec2f) -> Vec<Widget> {
		let ws = self.window_size();
		let mut widgets = Vec::new();

		let extra_draw = |i: usize| -> Option<DrawType> {
			match self.menu_state {
				MenuState::Attack(Some(j)) if i == j => {
					Some(Color::rgba(255, 0, 0, 20).into())
				},
				MenuState::DropChooseDir(ref indices) if indices.contains(&i) => {
					Some(Color::rgba(00, 0, 255, 20).into())
				}
				_ => None,
			}
		};

		let on_click = |i| match self.menu_state {
			MenuState::Attack(_) => {
				vec![MenuCommand::StateChange(MenuState::Attack(Some(i)))]
			},
			MenuState::ExecItem => {
				let cmd = UnitCommand::ExecItem(i);
				vec![
					MenuCommand::Command(Command::UnitCommand { command: cmd, pos: self.cursor }),
					MenuCommand::StateChange(MenuState::Normal),
				]
			},
			MenuState::DropChooseItem => {
				let mut hs = HashSet::new();
				hs.insert(i);
				vec![MenuCommand::StateChange(MenuState::DropChooseDir(hs))]
			}
			MenuState::DropChooseDir(ref current_indices) => {
				let mut current_indices = current_indices.clone();
				if current_indices.contains(&i) {
					current_indices.remove(&i);
				} else {
					current_indices.insert(i);
				}
				vec![MenuCommand::StateChange(MenuState::DropChooseDir(current_indices))]
			}
			_ => Vec::new(),
		};

		for (i, item) in u.inventory.iter().enumerate() {
			if let Some(dt) = extra_draw(i) {
				widgets.push(
					Widget {
						pos: ws * (offset + (0.03 * i as f32 + 0.01, 0.0)),
						size: ws * (0.025, 0.025),
						draw_type: dt,
						on_click: vec![],
						hotkey: None,
					});
			}

			widgets.push(
				Widget {
					pos: ws * (offset + (0.03 * i as f32 + 0.01, 0.0)),
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
