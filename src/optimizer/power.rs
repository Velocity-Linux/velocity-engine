use crate::config::types::ProfileConfig;
use crate::error::EngineError;
use crate::optimizer::{OptimizationState, Optimizer};

#[derive(Debug, Clone, Default)]
pub struct PowerOptimizer;

#[async_trait::async_trait]
impl Optimizer for PowerOptimizer {
    async fn apply(
        &mut self,
        profile: &ProfileConfig,
        _pids: &[u32],
        state: &mut OptimizationState,
    ) -> crate::error::Result<()> {
        if let Some(power_profile) = &profile.power_profile {
            let current = Self::get_current_power_profile();
            if current.as_deref() != Some(power_profile) {
                Self::set_power_profile(power_profile)
                    .map_err(|e| EngineError::Optimizer(format!("Cannot set power profile: {}", e)))?;
                state.power_profile = current;
                info!("Power profile set to {}", power_profile);
            }
        }
        Ok(())
    }

    async fn restore(&self, state: &OptimizationState) -> crate::error::Result<()> {
        if let Some(power_profile) = &state.power_profile {
            Self::set_power_profile(power_profile)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore power profile: {}", e)))?;
            info!("Power profile restored to {}", power_profile);
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "power"
    }
}

impl PowerOptimizer {
    fn get_current_power_profile() -> Option<String> {
        crate::system::SystemUtils::read_file("/var/lib/velocity-engine/power-profile")
            .or_else(|| crate::system::SystemUtils::run_command("powerprofilesctl", &["get"]).ok())
    }

    fn set_power_profile(profile: &str) -> Result<(), String> {
        crate::system::SystemUtils::run_command("powerprofilesctl", &["set", profile])
            .map(|_| ())
            .or_else(|_| {
                let fallback_path = "/var/lib/velocity-engine/power-profile";
                crate::system::SystemUtils::write_file(fallback_path, profile)
            })
    }
}
