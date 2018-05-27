use sfml::system::Vector2u;

use world::World;
use command::Command;
use view::View;
use misc::Direction;

impl World {
	pub fn exec(&mut self, command: Command, view: &mut View) {
		match command {
			Command::Move { from, direction } => self.exec_move(from, direction, view),
			Command::NextTurn => self.exec_next_turn(),
		}
	}

	fn exec_move(&mut self, from: Vector2u, direction: Direction, view: &mut View) {
		let to = direction.plus_vector(from);
		if let Some(mut unit) = self.get_unit(from).cloned() {
			if let Some(defending_unit) = self.get_unit(to).cloned() {
				// TODO
			} else {
				let stamina_cost = self.get_tile(from).get_stamina_cost() + self.get_tile(to).get_stamina_cost();
				if unit.stamina > stamina_cost {
					unit.stamina -= stamina_cost;
					self.unitmap[to.x as usize][to.y as usize] = Some(unit);
					self.unitmap[from.x as usize][from.y as usize] = None;
					view.marked_tile = to;
				}
			}
		}
	}

	fn exec_next_turn(&mut self) {
		self.active_player = 1 - self.active_player;

		if self.active_player == 0 {
			self.reset_turn();
		}
	}
}
