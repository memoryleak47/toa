use crate::*;

impl App {
	pub fn handle_hotkey(&mut self, k: Key) {
		match k {
			Key::Q => self.window.close(),
			Key::Escape => self.menu_state = MenuState::Normal,

			Key::W => self.apply_menu_commands(self.move_command(Direction::Up)),
			Key::A => self.apply_menu_commands(self.move_command(Direction::Left)),
			Key::S => self.apply_menu_commands(self.move_command(Direction::Down)),
			Key::D => self.apply_menu_commands(self.move_command(Direction::Right)),

			Key::Up => self.apply_menu_commands(self.cursor_move_command(Direction::Up)),
			Key::Left => self.apply_menu_commands(self.cursor_move_command(Direction::Left)),
			Key::Down => self.apply_menu_commands(self.cursor_move_command(Direction::Down)),
			Key::Right => self.apply_menu_commands(self.cursor_move_command(Direction::Right)),
			_ => {
				for w in self.generate_widgets() { // TODO this is not nice for performance!
					if w.hotkey == Some(k) {
						self.apply_menu_commands(w.on_click.clone());
					}
				}
			},
		}
	}

	fn move_command(&self, d: Direction) -> Vec<MenuCommand> {
		let cmd = UnitCommand::Move(d);
		let cmd = Command::UnitCommand { command: cmd, pos: self.cursor };
		let cmd = MenuCommand::Command(cmd);
		if let Some(new_cursor) = self.cursor.map(|x| x + *d) {
			vec![cmd, MenuCommand::Cursor(new_cursor)]
		} else {
			vec![]
		}
	}

	fn cursor_move_command(&self, d: Direction) -> Vec<MenuCommand> {
		if let Some(new_cursor) = self.cursor.map(|x| x + *d) {
			vec![MenuCommand::Cursor(new_cursor)]
		} else {
			vec![]
		}
	}

}

pub fn numeric_hotkey(n: usize) -> Option<Key> {
	match n {
		1 => Some(Key::Num1),
		2 => Some(Key::Num2),
		3 => Some(Key::Num3),
		4 => Some(Key::Num4),
		5 => Some(Key::Num5),
		6 => Some(Key::Num6),
		7 => Some(Key::Num7),
		8 => Some(Key::Num8),
		9 => Some(Key::Num9),
		10 => Some(Key::Num0),
		_ => None,
	}
}