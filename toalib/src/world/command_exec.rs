use std::mem;

use crate::vec::Pos;
use crate::item::ItemClass;
use crate::command::{Command, UnitCommand};
use crate::vec::Direction;
use crate::world::World;
use crate::aim::Aim;
use crate::world::unitmap::Unit;
use crate::world::buildingmap::BuildingClass;
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

	fn exec_unit_command(&mut self, pos: Pos, command: &UnitCommand) {
		let s = command.get_stamina_cost(pos, self);
		for u in self.unitmap.get_mut(pos).iter_mut() {
			u.stamina -= s as i32;
		}
			
		match command {
			&UnitCommand::Move(direction) => self.exec_move(pos, direction),
			&UnitCommand::Attack(ref aim) => self.exec_attack(pos, aim),
			&UnitCommand::Build(class) => self.exec_build(pos, class),
			&UnitCommand::Work => self.exec_work(pos),
			&UnitCommand::UnrefinedWork => self.exec_unrefined_work(pos),
			&UnitCommand::DropItem(i, dir) => self.exec_drop_item(pos, i, dir),
			&UnitCommand::TakeItem(i) => self.exec_take_item(pos, i),
			&UnitCommand::BurnBuilding => self.exec_discard_building(pos),
			&UnitCommand::Craft(ic) => self.exec_craft_item_class(ic, pos),
			&UnitCommand::ChangeMainItem(opt_index) => self.exec_change_main_item(opt_index, pos),
			&UnitCommand::ExecItem(i) => self.exec_exec_item(i, pos),
		}
	}

	fn exec_move(&mut self, from: Pos, direction: Direction) {
		let to = from.map(|x| x + *direction).unwrap();

		let mut tmp: Option<Unit> = None;
		mem::swap(&mut tmp, self.unitmap.get_mut_raw(from));
		mem::swap(&mut tmp, self.unitmap.get_mut_raw(to));
	}

	fn exec_attack(&mut self, pos: Pos, aim: &Aim) {
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

	fn exec_build(&mut self, at: Pos, class: BuildingClass) {
		let b = class.get_build_property().unwrap().build;
		self.buildingmap.set(at, Some((b)()));


		let cost = class.get_build_property()
				.unwrap()
				.item_cost;
		self.unitmap.get_mut(at).unwrap()
			.inventory.reduce(cost);
	}

	fn exec_work(&mut self, at: Pos) {
		let mut tmp_opt: Option<_> = None;
		mem::swap(&mut tmp_opt, self.buildingmap.get_mut_raw(at));
		tmp_opt.iter_mut()
			.for_each(|b| b.work(self, at));
		if self.buildingmap.get(at).is_none() {
			self.buildingmap.set(at, tmp_opt);
		}
	}

	fn exec_unrefined_work(&mut self, at: Pos) {
		let item_class = self.terrainmap.get(at).get_item_class();
		let u = self.unitmap.get_mut(at).unwrap();
		u.inventory.push(item_class.build());
	}

	fn exec_drop_item(&mut self, at: Pos, i: usize, opt_dir: Option<Direction>) {
		let droppos = opt_dir.map(|dir| {
			at.map(|x| x + *dir).unwrap()
		}).unwrap_or(at);

		let item = self.unitmap.get_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.remove(i);

		self.itemmap.get_mut(droppos)
			.push(item);
	}

	fn exec_take_item(&mut self, at: Pos, i: usize) {
		let item = self.itemmap.get_mut(at)
			.get_item_vec()
			.remove(i);
		self.unitmap.get_mut(at)
			.unwrap()
			.inventory
			.get_item_vec()
			.push(item);
	}

	fn exec_discard_building(&mut self, at: Pos) {
		self.buildingmap.set(at, None);
	}

	fn exec_craft_item_class(&mut self, ic: ItemClass, at: Pos) {
		let unit = self.unitmap.get_mut(at).unwrap();
		unit.inventory.reduce(ic.get_recipe().unwrap());
		unit.inventory.push(ic.build());
	}

	fn exec_change_main_item(&mut self, opt_index: Option<usize>, at: Pos) {
		let unit = self.unitmap.get_mut(at).unwrap();

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

	fn exec_exec_item(&mut self, i: usize, at: Pos) {
		let item = self.unitmap.get_mut(at)
			.unwrap()
			.inventory
			.remove(i);
		item.exec(at, self);
	}
}
