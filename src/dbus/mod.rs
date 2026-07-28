use std::sync::Arc;
use tokio::sync::RwLock;
use zbus::interface;
use zbus::SignalContext;

use crate::config::types::{GameRule, ProfileConfig};
use crate::daemon::DaemonEngine;
use crate::error::{EngineError, Result};

pub struct DbusApi {
    engine: Arc<RwLock<DaemonEngine>>,
}

#[interface(name = "org.velocityos.Engine")]
impl DbusApi {
    async fn get_status(&self) -> String {
        let engine = self.engine.read().await;
        format!(
            "active_games={}, active_profile={}",
            engine.active_games_count().await,
            engine.active_profile().await
        )
    }

    async fn get_active_profile(&self) -> String {
        let engine = self.engine.read().await;
        engine.active_profile().await
    }

    async fn activate_profile(&self, profile: &str) -> zbus::fdo::Result<()> {
        let mut engine = self.engine.write().await;
        engine.activate_profile(profile).await.map_err(|e| {
            zbus::fdo::Error::Failed(format!("{}", e))
        })?;
        Ok(())
    }

    async fn list_games(&self) -> Vec<String> {
        let engine = self.engine.read().await;
        engine.list_games().await
    }

    async fn list_profiles(&self) -> Vec<String> {
        let engine = self.engine.read().await;
        engine.list_profiles().await
    }

    async fn reload_configuration(&self) -> zbus::fdo::Result<()> {
        let mut engine = self.engine.write().await;
        engine.reload_config().await.map_err(|e| {
            zbus::fdo::Error::Failed(format!("{}", e))
        })?;
        Ok(())
    }

    async fn get_applied_optimizations(&self) -> Vec<String> {
        let engine = self.engine.read().await;
        engine.get_applied_optimizations().await
    }

    async fn game_started(&self, game_name: &str, profile: &str) {
        let mut engine = self.engine.write().await;
        if let Err(e) = engine.game_started(game_name, profile).await {
            warn!("Failed to apply optimizations for {}: {}", game_name, e);
        }
    }

    async fn game_stopped(&self, game_name: &str) {
        let mut engine = self.engine.write().await;
        if let Err(e) = engine.game_stopped(game_name).await {
            warn!("Failed to restore optimizations after {}: {}", game_name, e);
        }
    }
}

pub async fn start_dbus(engine: Arc<RwLock<DaemonEngine>>, bus_name: &str) -> Result<zbus::Connection> {
    info!("Starting D-Bus API on {}", bus_name);
    let connection = zbus::Connection::system()
        .await
        .map_err(|e| EngineError::DBus(format!("Cannot connect to D-Bus: {}", e)))?;

    let api = DbusApi { engine };
    connection
        .object_server()
        .at("/org/velocityos/Engine", api)
        .await
        .map_err(|e| EngineError::DBus(format!("Cannot register D-Bus object: {}", e)))?;

    connection
        .request_name(bus_name)
        .await
        .map_err(|e| EngineError::DBus(format!("Cannot request D-Bus name: {}", e)))?;

    info!("D-Bus API started successfully");
    Ok(connection)
}
