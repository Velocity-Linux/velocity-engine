// Copyright (C) 2026 Velocity OS Team
// This file is part of Velocity Engine, licensed under GPLv3.
// See LICENSE file in the project root for details.

pub mod config;
pub mod daemon;
pub mod dbus;
pub mod detector;
pub mod error;
pub mod logging;
pub mod optimizer;
pub mod profiles;
pub mod system;

pub use config::{Config, ConfigLoader};
pub use daemon::{DaemonEngine, DaemonService};
pub use dbus::DbusApi;
pub use detector::GameDetector;
pub use error::{EngineError, Result};
pub use optimizer::OptimizerManager;
pub use profiles::ProfileManager;
pub use system::SystemUtils;
