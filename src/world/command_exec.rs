use std::mem;

use sfml::system::Vector2u;

use item::ItemClass;
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
			u.stamina -= s as i32;
		}
			
		match command {
			&UnitCommand::Move(direction) => self.exec_move(pos, direction),
			&UnitCommand::Attack(to) => self.exec_attack(pos, to),
			&UnitCommand::Build(class)  => self.exec_build(pos, class),
			&UnitCommand::Work => self.exec_work(pos),
			&UnitCommand::UnrefinedWork => self.exec_unrefined_work(pos),
			&UnitCommand::DropItem(i) => self.exec_drop_item(pos, i),
			&UnitCommand::TakeItem(i) => self.exec_take_item(pos, i),
			&UnitCommand::BurnBuilding => self.exec_discard_building(pos),
			&UnitCommand::Craft(ic) => self.exec_craft_item_class(ic,  pos),
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

		self.get_unit_mut(at).unwrap()
			.inventory.reduce(class.get_build_item_cost());
	}

	fn exec_work(&mut self, at: Vector2u) {
		let mut tmp_opt: Option<Box<_>> = None;
		mem::swap(&mut tmp_opt, &mut self.buildingmap[at.x as usize][at.y as usize]);
		tmp_opt.iter_mut()
			.for_each(|b| b.work(self, at));
		if self.get_building(at).is_none() {
			self.set_building(at, tmp_opt);
		}
	}

	fn exec_unrefined_work(&mut self, at: Vector2u) {
		let item_class = self.get_terrain(at).get_item_class();
		let u = self.get_unit_mut(at).unwrap();
		u.inventory.push(item_class.build());
	}

	fn exec_drop_item(&mut self, at: Vector2u, i: usize) {
		let item = self.get_unit_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.remove(i);

		self.get_inventory_mut(at)
			.push(item);
	}

	fn exec_take_item(&mut self, at: Vector2u, i: usize) {
		let item = self.get_inventory_mut(at)
			.get_item_vec()
			.remove(i);
		self.get_unit_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.push(item);
	}

	fn exec_discard_building(&mut self, at: Vector2u) {
		self.set_building(at, None);
	}

	fn exec_craft_item_class(&mut self, ic: &'static ItemClass, at: Vector2u) {
		let mut unit = self.get_unit_mut(at).unwrap();
		unit.inventory.reduce(ic.get_recipe().unwrap());
		unit.inventory.push(ic.build());
	}
}
