use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::config::types::LoggingConfig;
use crate::error::Result;

pub fn init(config: &LoggingConfig) -> Result<()> {
    let filter = EnvFilter::try_new(&config.level).unwrap_or_else(|_| EnvFilter::new("info"));
    init_stderr(filter, &config.format)?;
    info!("Logging initialized at level: {}", config.level);
    Ok(())
}

fn init_stderr(filter: EnvFilter, format: &str) -> Result<()> {
    if format == "json" {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .json()
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| crate::error::EngineError::System(format!("Cannot init logging: {}", e)))?;
    } else {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| crate::error::EngineError::System(format!("Cannot init logging: {}", e)))?;
    }
    Ok(())
}
