use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub const COLORS: [(u8, u8, u8); 4] = [(200, 0, 0), (70, 200, 70), (0, 60, 220), (200, 170, 0)];

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct PlayerID(pub usize);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Team(pub usize);

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerPool {
    players: HashMap<PlayerID, Team>,
}

impl Display for PlayerID {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(formatter)
    }
}

impl PlayerID {
    pub fn get_color(self) -> (u8, u8, u8) {
        COLORS[self.0]
    }
}

impl PlayerPool {
    pub fn new() -> PlayerPool {
        PlayerPool {
            players: HashMap::new(),
        }
    }

    pub fn get_teams(&self) -> Vec<Team> {
        let mut teams = Vec::new();

        for (_, team) in self.players.iter() {
            if !teams.contains(team) {
                teams.push(*team);
            }
        }

        teams.sort_by_key(|&Team(x)| x);

        teams
    }

    pub fn get_ids_for_team(&self, team: Team) -> Vec<PlayerID> {
        self.players
            .iter()
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
        let new_id = self.players.iter().map(|(x, _)| x.0 + 1).max().unwrap_or(0);
        let player_id = PlayerID(new_id);
        assert!(new_id < COLORS.len(), "too many players!");
        self.players.insert(player_id, team);

        player_id
    }

    pub fn remove(&mut self, player_id: PlayerID) -> bool {
        self.players.remove(&player_id).is_some()
    }

    pub fn get_player_ids(&self) -> Vec<PlayerID> {
        let mut v: Vec<_> = self.players.keys().cloned().collect();

        v.sort_by_key(|&PlayerID(x)| x);

        v
    }

    pub fn change_team(&mut self, player_id: PlayerID, team: Team) -> bool {
        if self.players.contains_key(&player_id) {
            self.players.insert(player_id, team);
            true
        } else {
            false
        }
    }
}
