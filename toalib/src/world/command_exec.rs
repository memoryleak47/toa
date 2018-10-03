use std::mem;

use crate::vec::Vec2u;
use crate::item::ItemClass;
use crate::command::{Command, UnitCommand};
use crate::misc::Direction;
use crate::world::World;
use crate::world::aim::Aim;
use crate::world::unitmap::Unit;
use crate::world::buildingmap::{BuildingClass, new_construction};
use crate::team::PlayerID;

impl World {
	pub fn checked_exec(&mut self, player_id: PlayerID, command: &Command) -> bool {
		if !self.is_valid_command(player_id, command) { return false; }

		self.exec(player_id, command);

		return true;
	}

	fn exec(&mut self, player_id: PlayerID, command: &Command) {
		match command {
			&Command::NextTurn => self.exec_next_turn(player_id),
			&Command::UnitCommand { pos, ref command } => self.exec_unit_command(pos, command),
		}
	}

	fn exec_unit_command(&mut self, pos: Vec2u, command: &UnitCommand) {
		let s = command.get_stamina_cost(pos, self);
		for u in self.unitmap[index2d!(pos.x, pos.y)].iter_mut() {
			u.stamina -= s as i32;
		}
			
		match command {
			&UnitCommand::Move(direction) => self.exec_move(pos, direction),
			&UnitCommand::Attack(ref aim) => self.exec_attack(pos, aim),
			&UnitCommand::Build(class)  => self.exec_build(pos, class),
			&UnitCommand::Work => self.exec_work(pos),
			&UnitCommand::UnrefinedWork => self.exec_unrefined_work(pos),
			&UnitCommand::DropItem(i) => self.exec_drop_item(pos, i),
			&UnitCommand::TakeItem(i) => self.exec_take_item(pos, i),
			&UnitCommand::BurnBuilding => self.exec_discard_building(pos),
			&UnitCommand::Craft(ic) => self.exec_craft_item_class(ic, pos),
			&UnitCommand::ChangeMainItem(opt_index) => self.exec_change_main_item(opt_index, pos),
			&UnitCommand::ExecItem(i) => self.exec_exec_item(i, pos),
		}
	}

	fn exec_move(&mut self, from: Vec2u, direction: Direction) {
		let x1 = from.x as usize;
		let y1 = from.y as usize;

		let p = direction.plus_vector(from);

		let x2 = p.x as usize;
		let y2 = p.y as usize;

		let mut tmp: Option<Unit> = None;
		mem::swap(&mut tmp, &mut self.unitmap[index2d!(x1, y1)]);
		mem::swap(&mut tmp, &mut self.unitmap[index2d!(x2, y2)]);
	}

	fn exec_attack(&mut self, pos: Vec2u, aim: &Aim) {
		aim.exec(pos, self);
	}

	fn exec_next_turn(&mut self, player_id: PlayerID) {
		let current_team = self.pool.get_team_of(self.active_player_ids[0]);

		self.active_player_ids.retain(|x| *x != player_id);
		if self.active_player_ids.is_empty() {
			let next_team = self.pool.get_next_team(current_team);
			self.active_player_ids = self.pool.get_ids_for_team(next_team);
			if next_team == self.pool.get_starting_team() {
				self.reset_turn();
			}
			self.on_turn_start();
		}
	}

	fn exec_build(&mut self, at: Vec2u, class: BuildingClass) {
		self.buildingmap[index2d!(at.x, at.y)] = Some(new_construction(class));


		let cost = class.get_build_property()
				.unwrap()
				.item_cost;
		self.get_unit_mut(at).unwrap()
			.inventory.reduce(cost);
	}

	fn exec_work(&mut self, at: Vec2u) {
		let mut tmp_opt: Option<_> = None;
		mem::swap(&mut tmp_opt, &mut self.buildingmap[index2d!(at.x, at.y)]);
		tmp_opt.iter_mut()
			.for_each(|b| b.work(self, at));
		if self.get_building(at).is_none() {
			self.set_building(at, tmp_opt);
		}
	}

	fn exec_unrefined_work(&mut self, at: Vec2u) {
		let item_class = self.get_terrain(at).get_item_class();
		let u = self.get_unit_mut(at).unwrap();
		u.inventory.push(item_class.build());
	}

	fn exec_drop_item(&mut self, at: Vec2u, i: usize) {
		let item = self.get_unit_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.remove(i);

		self.get_inventory_mut(at)
			.push(item);
	}

	fn exec_take_item(&mut self, at: Vec2u, i: usize) {
		let item = self.get_inventory_mut(at)
			.get_item_vec()
			.remove(i);
		self.get_unit_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.push(item);
	}

	fn exec_discard_building(&mut self, at: Vec2u) {
		self.set_building(at, None);
	}

	fn exec_craft_item_class(&mut self, ic: ItemClass, at: Vec2u) {
		let unit = self.get_unit_mut(at).unwrap();
		unit.inventory.reduce(ic.get_recipe().unwrap());
		unit.inventory.push(ic.build());
	}

	fn exec_change_main_item(&mut self, opt_index: Option<usize>, at: Vec2u) {
		let unit = self.get_unit_mut(at).unwrap();

		let mut opt = None;
		mem::swap(&mut opt, &mut unit.main_item);

		if let Some(x) = opt {
			unit.inventory.push(x);
		}
		if let Some(i) = opt_index {
			let item = unit.inventory.remove(i);
			unit.main_item = Some(item);
		}
	}

	fn exec_exec_item(&mut self, i: usize, at: Vec2u) {
		let item = self.get_unit_mut(at)
			.unwrap()
			.inventory
			.remove(i);
		item.exec(at, self);
	}
}
