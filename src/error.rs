use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("system error: {0}")]
    System(String),

    #[error("D-Bus error: {0}")]
    DBus(String),

    #[error("profile error: {0}")]
    Profile(String),

    #[error("optimizer error: {0}")]
    Optimizer(String),

    #[error("detector error: {0}")]
    Detector(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("plugin error: {0}")]
    Plugin(String),

    #[error("serialization error: {0}")]
    Serialization(#[from] toml::de::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("D-Bus proxy error: {0}")]
    Zbus(#[from] zbus::Error),
}

pub type Result<T> = std::result::Result<T, EngineError>;
