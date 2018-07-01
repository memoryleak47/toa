use std::mem;

use sfml::system::Vector2u;

use command::{Command, UnitCommand};
use misc::Direction;
use world::World;
use world::unitmap::Unit;
use world::buildingmap::BuildingClass;
use world::buildingmap::construction::Construction;

impl World {
	pub fn exec(&mut self, command: &Command) {
		assert!(self.is_valid_command(self.active_player, command));

		match command {
			&Command::NextTurn => self.exec_next_turn(),
			&Command::UnitCommand { pos, ref command } => self.exec_unit_command(pos, command),
		}
	}

	fn exec_unit_command(&mut self, pos: Vector2u, command: &UnitCommand) {
		let s = command.get_stamina_cost(pos, self);
		for u in self.unitmap[pos.x as usize][pos.y as usize].iter_mut() {
			u.stamina -= s;
		}
			
		match command {
			&UnitCommand::Move(direction) => self.exec_move(pos, direction),
			&UnitCommand::Attack(to) => self.exec_attack(pos, to),
			&UnitCommand::Build(class)  => self.exec_build(pos, class),
			&UnitCommand::Work => self.exec_work(pos),
			&UnitCommand::UnrefinedWork => self.exec_unrefined_work(pos),
		}
	}

	fn exec_move(&mut self, from: Vector2u, direction: Direction) {
		let x1 = from.x as usize;
		let y1 = from.y as usize;

		let p = direction.plus_vector(from);

		let x2 = p.x as usize;
		let y2 = p.y as usize;

		let mut tmp: Option<Unit> = None;
		mem::swap(&mut tmp, &mut self.unitmap[x1][y1]);
		mem::swap(&mut tmp, &mut self.unitmap[x2][y2]);
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
	}
}
