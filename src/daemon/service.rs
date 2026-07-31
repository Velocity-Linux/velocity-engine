use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{error, info};

use crate::daemon::engine::DaemonEngine;
use crate::dbus::start_dbus;
use crate::error::Result;

pub struct DaemonService {
    engine: Arc<RwLock<DaemonEngine>>,
    dbus_connection: Option<zbus::Connection>,
}

impl DaemonService {
    pub async fn new(config: crate::config::types::Config) -> Result<Self> {
        let engine = DaemonEngine::new(config).await?;
        let engine = Arc::new(RwLock::new(engine));

        Ok(Self {
            engine,
            dbus_connection: None,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let engine = self.engine.read().await;
        let dbus_name = engine.dbus_name().to_string();
        drop(engine);

        info!("Starting D-Bus service...");
        let connection = start_dbus(self.engine.clone(), &dbus_name).await?;
        self.dbus_connection = Some(connection);

        info!("Velocity Engine daemon started successfully");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Velocity Engine running");
        let poll_interval = {
            let engine = self.engine.read().await;
            engine.poll_interval()
        };

        let mut ticker = interval(Duration::from_millis(poll_interval));

        loop {
            ticker.tick().await;

            let result = self.tick().await;
            if let Err(e) = result {
                error!("Daemon tick error: {}", e);
            }
        }
    }

    async fn tick(&self) -> Result<()> {
        let engine = self.engine.read().await;
        engine.detect_and_optimize().await?;
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Velocity Engine");
        let engine = self.engine.read().await;
        engine.restore_all().await?;
        Ok(())
    }
}
