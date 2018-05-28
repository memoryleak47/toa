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

		let stamina_cost = self.get_terrain(from).get_stamina_cost() + self.get_terrain(to).get_stamina_cost();

		if let Some(mut unit) = self.get_unit(from).cloned() {
			if unit.stamina > stamina_cost {
				if let Some(mut defending_unit) = self.get_unit(to).cloned() {
					if defending_unit.owner != unit.owner {
						unit.stamina -= stamina_cost;

						defending_unit.health = defending_unit.health.saturating_sub(10);
						unit.health = unit.health.saturating_sub(10);
						// TODO remove dead units

						self.unitmap[from.x as usize][from.y as usize] = Some(unit);
						self.unitmap[to.x as usize][to.y as usize] = Some(defending_unit);
					}
				} else {
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
		self.on_turn_start();
	}
}
