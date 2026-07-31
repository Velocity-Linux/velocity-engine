use crate::config::types::Config;
use crate::error::{EngineError, Result};
use notify::{Event as NotifyEvent, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

pub struct ConfigLoader {
    path: PathBuf,
    config: RwLock<Config>,
    watcher: Option<RecommendedWatcher>,
}

impl ConfigLoader {
    pub async fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let config = Self::load_from_file(&path)?;
        let (tx, _rx) = channel::<()>();
        let mut watcher = RecommendedWatcher::new(
            move |res: std::result::Result<NotifyEvent, notify::Error>| {
                if let Err(e) = res {
                    error!("Config watcher error: {}", e);
                    return;
                }
                let _ = tx.send(());
            },
            notify::Config::default(),
        )
        .map_err(|e| EngineError::Config(format!("Failed to create watcher: {}", e)))?;

        if let Some(parent) = path.parent() {
            if let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive) {
                warn!("Cannot watch config dir: {}", e);
            }
        }

        Ok(Self {
            path,
            config: RwLock::new(config),
            watcher: Some(watcher),
        })
    }

    pub async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, Config> {
        self.config.read().await
    }

    pub async fn reload(&self) -> Result<()> {
        info!("Reloading configuration from {:?}", self.path);
        let new_config = Self::load_from_file(&self.path)?;
        *self.config.write().await = new_config;
        info!("Configuration reloaded successfully");
        Ok(())
    }

    pub async fn watch_for_changes(&self) {
        let (_tx, _rx) = channel::<()>();
        loop {
            if let Ok(_) = _rx.recv_timeout(Duration::from_millis(100)) {
                if let Err(e) = self.reload().await {
                    error!("Failed to reload config: {}", e);
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    fn load_from_file(path: &PathBuf) -> Result<Config> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| EngineError::Config(format!("Cannot read config file: {}", e)))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| EngineError::Config(format!("Cannot parse config: {}", e)))?;
        Self::validate(&config)?;
        Ok(config)
    }

    fn validate(config: &Config) -> Result<()> {
        if config.profiles.is_empty() {
            return Err(EngineError::Config("No profiles defined".to_string()));
        }
        for game in &config.games {
            if !config.profiles.contains_key(&game.profile) {
                return Err(EngineError::Config(format!(
                    "Game '{}' references unknown profile '{}'",
                    game.name, game.profile
                )));
            }
        }
        if config.daemon.poll_interval_ms < 100 {
            return Err(EngineError::Config(
                "poll_interval_ms must be >= 100".to_string(),
            ));
        }
        Ok(())
    }
}
