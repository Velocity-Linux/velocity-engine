use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::config::types::{Config, GameRule};
use crate::detector::GameDetector;
use crate::error::Result;
use crate::optimizer::OptimizerManager;
use crate::profiles::ProfileManager;
use crate::system::SystemUtils;

pub struct DaemonEngine {
    config: Config,
    profile_manager: ProfileManager,
    detector: GameDetector,
    optimizer: OptimizerManager,
    active_profile: RwLock<String>,
    active_games: RwLock<Vec<String>>,
    states: RwLock<Vec<(String, OptimizationState)>>,
}

impl DaemonEngine {
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing Velocity Engine daemon");

        let profile_manager = ProfileManager::new(config.profiles.clone(), config.games.clone());
        let detector = GameDetector::new(config.games.clone());
        let optimizer = OptimizerManager::new();

        Ok(Self {
            config,
            profile_manager,
            detector,
            optimizer,
            active_profile: RwLock::new("default".to_string()),
            active_games: RwLock::new(Vec::new()),
            states: RwLock::new(Vec::new()),
        })
    }

    pub async fn detect_and_optimize(&self) -> Result<()> {
        let detected = self.detector.check_games().await?;

        if detected.is_empty() {
            self.restore_if_needed().await?;
            return Ok(());
        }

        let mut active_games = self.active_games.write().await;
        let mut active_profile = self.active_profile.write().await;

        for game in detected {
            if !active_games.contains(&game.name) {
                active_games.push(game.name.clone());

                let profile = self
                    .profile_manager
                    .get_profile(&game.profile)
                    .ok_or_else(|| {
                        crate::error::EngineError::Profile(format!(
                            "Profile not found: {}",
                            game.profile
                        ))
                    })?;

                let pids = self.get_game_pids(&game).await;
                self.optimizer.apply_profile(profile, &pids).await?;

                let mut state = OptimizationState::default();
                self.optimizer.apply_profile(profile, &pids).await?;
                self.states.write().await.push((game.name.clone(), state));

                *active_profile = game.profile.clone();
                info!(
                    "Optimizations applied for {} with profile '{}'",
                    game.name, game.profile
                );
            }
        }

        Ok(())
    }

    pub async fn game_started(&self, game_name: &str, profile: &str) -> Result<()> {
        info!(
            "Game started via plugin: {} with profile '{}'",
            game_name, profile
        );
        let pids = self.get_pids_for_game_name(game_name).await;
        let profile_config = self.profile_manager.get_profile(profile).ok_or_else(|| {
            crate::error::EngineError::Profile(format!("Profile not found: {}", profile))
        })?;

        self.optimizer.apply_profile(profile_config, &pids).await?;
        *self.active_profile.write().await = profile.to_string();
        Ok(())
    }

    pub async fn game_stopped(&self, game_name: &str) -> Result<()> {
        info!("Game stopped via plugin: {}", game_name);
        self.restore_game_state(game_name).await?;
        Ok(())
    }

    pub async fn restore_all(&self) -> Result<()> {
        warn!("Restoring all optimizations");
        self.optimizer.restore().await?;
        *self.active_games.write().await = Vec::new();
        *self.active_profile.write().await = "default".to_string();
        *self.states.write().await = Vec::new();
        Ok(())
    }

    pub async fn reload_config(&mut self) -> Result<()> {
        info!("Reloading configuration");
        // In real implementation, reload from config loader
        Ok(())
    }

    pub async fn activate_profile(&self, profile: &str) -> Result<()> {
        info!("Manually activating profile: {}", profile);
        let profile_config = self.profile_manager.get_profile(profile).ok_or_else(|| {
            crate::error::EngineError::Profile(format!("Profile not found: {}", profile))
        })?;

        let pids = self.get_all_game_pids().await;
        self.optimizer.apply_profile(profile_config, &pids).await?;
        *self.active_profile.write().await = profile.to_string();
        Ok(())
    }

    pub async fn list_games(&self) -> Vec<String> {
        self.profile_manager
            .list_games()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub async fn list_profiles(&self) -> Vec<String> {
        self.profile_manager.list_profiles()
    }

    pub async fn active_profile(&self) -> String {
        self.active_profile.read().await.clone()
    }

    pub async fn active_games_count(&self) -> usize {
        self.active_games.read().await.len()
    }

    pub async fn get_applied_optimizations(&self) -> Vec<String> {
        self.optimizer.list_optimizers()
    }

    pub fn dbus_name(&self) -> &str {
        &self.config.daemon.dbus_name
    }

    pub fn poll_interval(&self) -> u64 {
        self.config.daemon.poll_interval_ms
    }

    async fn restore_if_needed(&self) -> Result<()> {
        let active_games = self.active_games.read().await;
        if active_games.is_empty() {
            let states = self.states.read().await;
            if !states.is_empty() {
                drop(active_games);
                drop(states);
                self.restore_all().await?;
            }
        }
        Ok(())
    }

    async fn get_game_pids(&self, game: &GameRule) -> Vec<u32> {
        let mut pids = Vec::new();
        for process in &game.processes {
            pids.extend(crate::system::SystemUtils::find_pids_by_name(process));
        }
        pids
    }

    async fn get_pids_for_game_name(&self, name: &str) -> Vec<u32> {
        let game = self.profile_manager.get_game_rule(name);
        if let Some(game) = game {
            self.get_game_pids(game).await
        } else {
            crate::system::SystemUtils::find_pids_by_name(name)
        }
    }

    async fn get_all_game_pids(&self) -> Vec<u32> {
        let mut pids = Vec::new();
        for game_name in self.active_games.read().await.iter() {
            pids.extend(self.get_pids_for_game_name(game_name).await);
        }
        pids
    }

    async fn restore_game_state(&self, game_name: &str) -> Result<()> {
        let mut active_games = self.active_games.write().await;
        active_games.retain(|g| g != game_name);

        let mut states = self.states.write().await;
        if let Some(pos) = states.iter().position(|(name, _)| name == game_name) {
            states.remove(pos);
        }

        if active_games.is_empty() && !states.is_empty() {
            self.optimizer.restore().await?;
            states.clear();
            *self.active_profile.write().await = "default".to_string();
        }

        Ok(())
    }
}
