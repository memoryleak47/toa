use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct PlayerID(usize);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Team {
	Red, 
	Blue
}

#[derive(Serialize, Deserialize)]
pub struct PlayerPool {
	players: HashMap<PlayerID, Team>,
}

impl Display for PlayerID {
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
		self.0.fmt(formatter)
	}
}

impl PlayerID {
	pub fn new(x: usize) -> PlayerID {
		PlayerID(x)
	}
}

impl PlayerPool {
	pub fn new() -> PlayerPool {
		PlayerPool {
			players: HashMap::new(),
		}
	}

	pub fn get_teams(&self) -> Vec<Team> {
		self.players.iter()
			.map(|(_, y)| *y)
			.collect()
	}

	pub fn get_ids_for_team(&self, team: Team) -> Vec<PlayerID> {
		self.players.iter()
			.filter(|(_, y)| **y == team)
			.map(|(x, _)| *x)
			.collect()
	}

	pub fn get_starting_team(&self) -> Team {
		self.get_teams()[0]
	}

	pub fn get_next_team(&self, team: Team) -> Team {
		let teams = self.get_teams();
		let old_index = teams.iter().position(|x| *x == team).unwrap();
		let new_index = (old_index + 1) % teams.len();

		teams[new_index]
	}

	pub fn get_team_of(&self, player: PlayerID) -> Team {
		self.players[&player]
	}

	pub fn add(&mut self, team: Team) -> PlayerID {
		let new_id = self.players.iter()
			.map(|(x, _)| x.0)
			.max()
			.unwrap() + 1;
		let player_id = PlayerID::new(new_id);
		self.players.insert(player_id, team);

		player_id
	}

	pub fn remove(&mut self, player_id: PlayerID) -> bool {
		self.players.remove(&player_id).is_some()
	}
}
