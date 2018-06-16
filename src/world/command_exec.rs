use sfml::system::Vector2u;

use world::World;
use command::Command;
use view::View;
use misc::Direction;
use world::buildingmap::{Building, BuildingPlan, BuildingKind};
use world::terrainmap::Terrain;

impl World {
	pub fn exec(&mut self, command: &Command, view: &mut View) {
		assert!(self.is_valid_command(self.active_player, command));

		match command {
			&Command::Move { from, direction } => self.exec_move(from, direction, view),
			&Command::Attack { from, to } => self.exec_attack(from, to),
			&Command::NextTurn => self.exec_next_turn(),
			&Command::Build { at, plan } => self.exec_build(at, plan),
			&Command::Work { at } => self.exec_work(at),
			
		}
	}

	fn exec_move(&mut self, from: Vector2u, direction: Direction, view: &mut View) {
		let to = direction.plus_vector(from);

		let stamina_cost = self.get_terrain(from).get_stamina_cost() + self.get_terrain(to).get_stamina_cost();

		// TODO this should not do an attack!
		if let Some(mut unit) = self.get_unit(from).cloned() {
			if unit.stamina > stamina_cost {
				if let Some(mut defending_unit) = self.get_unit(to).cloned() {
					if defending_unit.owner != unit.owner {
						unit.stamina -= stamina_cost;

						defending_unit.health = defending_unit.health.saturating_sub(10);
						unit.health = unit.health.saturating_sub(10);

						self.unitmap[from.x as usize][from.y as usize] = if unit.health > 0 { Some(unit) } else { None };
						self.unitmap[to.x as usize][to.y as usize] = if defending_unit.health > 0 { Some(defending_unit) } else { None };
					}
				} else {
					unit.stamina -= stamina_cost;
					self.unitmap[to.x as usize][to.y as usize] = Some(unit);
					self.unitmap[from.x as usize][from.y as usize] = None;
					view.main_cursor = to; // TODO this is hacky!
				}
			}
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

	fn exec_build(&mut self, at: Vector2u, plan: &'static BuildingPlan<'static>) {
		match plan.building.kind {
			BuildingKind::Farm { .. } => {
				if *self.get_terrain(at) != Terrain::GRASS { return; }
				let construction = BuildingKind::InConstruction { required_stamina: plan.required_stamina, building: plan.building.clone() };
				self.buildingmap[at.x as usize][at.y as usize] = Some(Building { health: 10, kind: construction })
			},
			_ => { /* TODO */ },
		}
	}

	fn exec_work(&mut self, at: Vector2u) {
		if !self.get_unit(at).is_some() || ! (self.get_unit(at).unwrap().stamina >= 10) || !self.get_building(at).is_some() { return; }

		if let Some(unit) = self.get_unit_mut(at) {
			unit.stamina -= 10;
		}
		if let Some(building) = self.get_building_mut(at) {
			building.work();
		}
	}
}
