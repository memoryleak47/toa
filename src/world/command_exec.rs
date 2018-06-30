use sfml::system::Vector2u;

use command::Command;
use misc::Direction;
use world::World;
use world::REQUIRED_UNREFINED_WORK_STAMINA;
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
			&Command::UnrefinedWork { at } => self.exec_unrefined_work(at),
		}
	}

	fn exec_move(&mut self, from: Vector2u, direction: Direction) {
		// TODO remove stamina, move unit
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

	fn exec_unrefined_work(&mut self, at: Vector2u) {
		let item_class = self.get_terrain(at).get_item_class();
		let mut u = self.get_unit_mut(at).unwrap();
		u.inventory.push(item_class.build());
		u.stamina = u.stamina.saturating_sub(REQUIRED_UNREFINED_WORK_STAMINA);
	}
}
