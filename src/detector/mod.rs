use std::collections::HashSet;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::info;

use crate::config::types::GameRule;
use crate::system::SystemUtils;

pub struct GameDetector {
    games: Vec<GameRule>,
    active_games: RwLock<HashSet<String>>,
    last_check: RwLock<Instant>,
}

impl GameDetector {
    pub fn new(games: Vec<GameRule>) -> Self {
        Self {
            games,
            active_games: RwLock::new(HashSet::new()),
            last_check: RwLock::new(Instant::now()),
        }
    }

    pub async fn check_games(&self) -> Result<Vec<GameRule>> {
        let mut detected = Vec::new();
        let mut active = self.active_games.write().await;
        let mut new_active = HashSet::new();

        for game in &self.games {
            for process in &game.processes {
                if SystemUtils::is_process_running(process) {
                    if !active.contains(&game.name) {
                        info!("Game detected: {} (process: {})", game.name, process);
                    }
                    new_active.insert(game.name.clone());
                    if !detected.iter().any(|g: &GameRule| g.name == game.name) {
                        detected.push(game.clone());
                    }
                    break;
                }
            }
        }

        let exited: Vec<String> = active.difference(&new_active).cloned().collect();
        for game_name in &exited {
            info!("Game exited: {}", game_name);
        }

        *active = new_active;
        *self.last_check.write().await = Instant::now();

        Ok(detected)
    }

    pub async fn get_active_games(&self) -> Vec<String> {
        let active = self.active_games.read().await;
        active.iter().cloned().collect()
    }

    pub async fn is_game_active(&self, name: &str) -> bool {
        let active = self.active_games.read().await;
        active.contains(name)
    }

    pub fn games(&self) -> &[GameRule] {
        &self.games
    }
}
