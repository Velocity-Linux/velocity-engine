use std::collections::HashMap;

use crate::config::types::{GameRule, ProfileConfig};
use crate::error::Result;

pub struct ProfileManager {
    profiles: HashMap<String, ProfileConfig>,
    games: HashMap<String, GameRule>,
}

impl ProfileManager {
    pub fn new(profiles: HashMap<String, ProfileConfig>, games: Vec<GameRule>) -> Self {
        let games_map: HashMap<String, GameRule> = games.into_iter().map(|g| (g.name.clone(), g)).collect();
        Self { profiles, games: games_map }
    }

    pub fn get_profile(&self, name: &str) -> Option<&ProfileConfig> {
        self.profiles.get(name)
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn get_game_rule(&self, name: &str) -> Option<&GameRule> {
        self.games.get(name)
    }

    pub fn list_games(&self) -> Vec<&String> {
        self.games.keys().collect()
    }

    pub fn find_profile_for_process(&self, process_name: &str) -> Option<&ProfileConfig> {
        for game in self.games.values() {
            if game.processes.iter().any(|p| p == process_name) {
                return self.profiles.get(&game.profile);
            }
        }
        None
    }
}
