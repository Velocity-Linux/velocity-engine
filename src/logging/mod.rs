use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_journald::JournaldWriter;

use crate::config::types::LoggingConfig;
use crate::error::Result;

pub fn init(config: &LoggingConfig) -> Result<()> {
    let filter = EnvFilter::try_new(&config.level).unwrap_or_else(|_| EnvFilter::new("info"));

    if config.journald {
        match JournaldWriter::new() {
            Ok(journald_writer) => {
                let subscriber = tracing_subscriber::registry()
                    .with(filter)
                    .with(tracing_journald::layer().with_writer(journald_writer));

                if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
                    warn!(
                        "Cannot set journald subscriber: {}, falling back to stderr",
                        e
                    );
                    init_stderr(filter, config.format)?;
                }
            }
            Err(e) => {
                warn!("Cannot connect to journald: {}, using stderr", e);
                init_stderr(filter, config.format)?;
            }
        }
    } else {
        init_stderr(filter, config.format)?;
    }

    info!("Logging initialized at level: {}", config.level);
    Ok(())
}

fn init_stderr(filter: EnvFilter, format: &str) -> Result<()> {
    let fmt_layer = if format == "json" {
        fmt::layer().json()
    } else {
        fmt::layer()
    };

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer.with_writer(std::io::stderr));

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| crate::error::EngineError::System(format!("Cannot init logging: {}", e)))?;

    Ok(())
}
