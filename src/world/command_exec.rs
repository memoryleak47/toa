use sfml::system::Vector2u;

use world::World;
use command::Command;
use misc::Direction;
use world::buildingmap::BuildingClass;
use world::buildingmap::construction::Construction;

impl World {
	pub fn exec(&mut self, command: &Command) {
		assert!(self.is_valid_command(self.active_player, command));

		match command {
			&Command::Move { from, direction } => self.exec_move(from, direction),
			&Command::Attack { from, to } => self.exec_attack(from, to),
			&Command::NextTurn => self.exec_next_turn(),
			&Command::Build { at, class }  => self.exec_build(at, class),
			&Command::Work { at } => self.exec_work(at),
		}
	}

	fn exec_move(&mut self, from: Vector2u, direction: Direction) {
		let to = direction.plus_vector(from);

		let stamina_cost = self.required_walk_stamina(from, direction);

		if let Some(mut unit) = self.get_unit(from).cloned() {
			unit.stamina -= stamina_cost;
			self.unitmap[to.x as usize][to.y as usize] = Some(unit);
			self.unitmap[from.x as usize][from.y as usize] = None;
		}
	}

	fn exec_attack(&mut self, from: Vector2u, to: Vector2u) {
		unimplemented!()
	}

	fn exec_next_turn(&mut self) {
		self.active_player = 1 - self.active_player;

		if self.active_player == 0 {
			self.reset_turn();
		}
		self.on_turn_start();
	}

	fn exec_build(&mut self, at: Vector2u, class: &'static BuildingClass) {
		let construction = Construction::new(class);
		let boxed = Box::new(construction);
		self.buildingmap[at.x as usize][at.y as usize] = Some(boxed);
	}

	fn exec_work(&mut self, at: Vector2u) {
		let f = self.get_building_mut(at)
			.unwrap()
			.get_class()
			.get_work_fn();
		f(self, at);
	}
}
