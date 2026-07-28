use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub daemon: DaemonConfig,
    pub profiles: HashMap<String, ProfileConfig>,
    pub games: Vec<GameRule>,
    pub plugins: PluginConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub config_path: String,
    pub poll_interval_ms: u64,
    pub restore_timeout_ms: u64,
    pub enable_plugins: bool,
    pub dbus_name: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            config_path: "/etc/velocity-engine/default.toml".to_string(),
            poll_interval_ms: 1000,
            restore_timeout_ms: 5000,
            enable_plugins: true,
            dbus_name: "org.velocityos.Engine".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub cpu_governor: Option<String>,
    pub cpu_affinity: Option<Vec<usize>>,
    pub process_priority: Option<i32>,
    pub io_priority: Option<String>,
    pub power_profile: Option<String>,
    pub scheduler: Option<String>,
    pub tweaks: HashMap<String, String>,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            cpu_governor: None,
            cpu_affinity: None,
            process_priority: None,
            io_priority: None,
            power_profile: None,
            scheduler: None,
            tweaks: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRule {
    pub name: String,
    pub processes: Vec<String>,
    pub profile: String,
    pub custom_affinity: Option<Vec<usize>>,
    pub custom_priority: Option<i32>,
    pub custom_io_priority: Option<String>,
    pub plugins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: Vec<String>,
    pub disabled: Vec<String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: vec![],
            disabled: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub journald: bool,
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            journald: true,
            format: "text".to_string(),
        }
    }
}
