use crate::*;

impl App {
	pub fn handle_hotkey(&mut self, k: Key) {
		match k {
			Key::Q => self.window.close(),
			Key::Escape => self.menu_state = MenuState::Normal,
			Key::Return => self.apply_menu_commands(self.main_button_cmds()),

			Key::W => self.apply_menu_commands(self.move_command(Direction::Up)),
			Key::A => self.apply_menu_commands(self.move_command(Direction::Left)),
			Key::S => self.apply_menu_commands(self.move_command(Direction::Down)),
			Key::D => self.apply_menu_commands(self.move_command(Direction::Right)),

			Key::Up => self.apply_menu_commands(self.cursor_move_command(Direction::Up)),
			Key::Left => self.apply_menu_commands(self.cursor_move_command(Direction::Left)),
			Key::Down => self.apply_menu_commands(self.cursor_move_command(Direction::Down)),
			Key::Right => self.apply_menu_commands(self.cursor_move_command(Direction::Right)),
			_ => {},
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

