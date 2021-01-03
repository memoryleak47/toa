use crate::*;

impl World {
	fn is_valid_unit_command(&self, player: PlayerID, pos: Pos, command: &UnitCommand) -> Result<(), String> {
		let unit = self.unitmap.get(pos).ok_or_else(|| "No unit selected.".to_owned())
		.and_then(|x| if x.owner == player { Ok(x) } else { Err("The selected unit is not your unit.".to_owned()) })
		.and_then(|x| if x.stamina > 0 { Ok(x) } else { Err("No Stamina.".to_owned()) })?;
		match command {
			UnitCommand::Move(direction) => {
				let to = pos.map(|x| x + **direction).ok_or_else(|| "Cannot move out of map.".to_owned())?;

				if self.unitmap.get(to).is_some() { Err("Cannot move to occupied tile.".to_owned())? };
				self.allowed_to_go_to(pos, to)?;
				Ok(())
			},
			UnitCommand::Attack(weapon_id, _) => {
				if !weapon_id.map(|i| unit.inventory.has_index(i)).unwrap_or(true) { Err("Item index out of range while attacking.".to_owned())? };
				Ok(())
			},
			UnitCommand::Build(class) => {
				let prop = class.get_build_property().ok_or_else(|| format!("The building {} cannot be build.", class.get_name()))?;
				let req_terrain_opt = prop.required_terrain;
				if self.buildingmap.get(pos).is_some() { Err("Cannot build where a building already exists.".to_owned())?; }
				let terrain = self.terrainmap.get(pos);
				if terrain.prevents_building() { Err(format!("You cannot build on {}.", terrain.str()))?; }
				if !unit.inventory.contains_all(prop.item_cost) { Err(format!("You cannot build {}, as you don't have the full recipe.", class.get_name()))?; }
				if let Some(req_terrain) = req_terrain_opt {
					if req_terrain != *terrain { Err(format!("Building {} can only be built on {}.", class.get_name(), req_terrain.str()))?; }
				}
				Ok(())
			},
			UnitCommand::Work => {
				let b = self.buildingmap.get(pos).ok_or_else(|| "Here is no building to work on.".to_owned())?;
				if !b.is_workable(self, pos) { Err(format!("You cannot work on {}", b.get_class().get_name()))?; }
				Ok(())
			},
			UnitCommand::TerrainWork => {
				let terrain = self.terrainmap.get(pos);
				let b = self.buildingmap.get(pos);
				if terrain.terrain_work_stats(b).is_none() { Err(format!("You cannot work on {} (without the right building).", terrain.str()))?; }
				Ok(())
			}
			UnitCommand::DropItem(i, opt_dir) => {
				if !(unit.inventory.iter().len() > *i) { Err("DropItem index out of range.".to_owned())?; }
				if !opt_dir.map(|dir| {
					pos.map(|x| x + *dir).is_some()
				}).unwrap_or(true) { Err("Cannot drop item out of map.".to_owned())?; }
				Ok(())
			},
			UnitCommand::TakeItem(i) => {
				if !(self.itemmap.get(pos)
					.iter()
					.len() > *i) { Err("TakeItem index out of range".to_owned())?; }
				Ok(())
			},
			UnitCommand::BurnBuilding => {
				let b = self.buildingmap.get(pos).ok_or_else(|| "Here is no building to burn.".to_owned())?;
				if !b.is_burnable(self, pos) { Err(format!("The building {} cannot be burned", b.get_class().get_name()))?; }
				Ok(())
			},
			UnitCommand::Craft(class) => {
				let recipe = class.get_recipe().ok_or_else(|| format!("Cannot craft {}.", class.get_name()) )?;
				if let Some(Building::Workshop(_)) = self.buildingmap.get(pos) {
						if !unit.inventory.contains_all(recipe) { Err(format!("You cannot craft {}, as you don't have the full recipe.", class.get_name()))?; }
				} else { Err("You can only craft on a Workshop.".to_owned())? }
				Ok(())
			},
			UnitCommand::ExecItem(i) => {
				if let Some(item) = unit.inventory.iter().nth(*i) {
					if !item.is_execable(pos, self) { Err(format!("Item {} is not executable here", item.get_class().get_name()))? }
				} else { Err("ExecItem index out of range".to_owned())? }
				Ok(())
			},
			UnitCommand::FarmFood => {
				if self.ready_to_spawn(player) {
					Err("FarmFood can not be executed when a unit can be spawned!".to_owned())?;
				}
				Ok(())
			},
			UnitCommand::SpawnUnit(d) => {
				if !self.ready_to_spawn(player) {
					Err("SpawnUnit can only be executed when enough food is collected!".to_owned())?;
				}
				let target = pos.map(|x| x + **d);
				match target {
					Some(p) => if self.unitmap.get(p).is_some() {
						Err("You can not spawn a unit there. There already is one!".to_owned())?;
					}
					None => Err("SpawnUnit: Out of range!")?,
				}
				Ok(())
			},
			UnitCommand::Idle => Ok(()),
		}
	}

	pub fn is_valid_command(&self, player: PlayerID, command: &Command) -> Result<(), String> {
		if !self.active_player_ids.contains(&player) { Err("It is not your turn.".to_owned())?; }

		match command {
			Command::NextTurn => Ok(()),
			Command::UnitCommand { ref command, pos } => self.is_valid_unit_command(player, *pos, command),
		}
	}

	fn allowed_to_go_to(&self, from: Pos, to: Pos) -> Result<(), String> {
		let player_id = self.unitmap.get(from).unwrap().owner;

		if self.terrainmap.get(to).is_blocking() {
			Err("The terrain is blocking.".to_string())?;
		}

		if self.buildingmap.get(to)
				.map(|b| b.is_blocking_against(player_id))
				.unwrap_or(false) {
			Err("A building is blocking.".to_string())?;
		}

		Ok(())
	}
}
