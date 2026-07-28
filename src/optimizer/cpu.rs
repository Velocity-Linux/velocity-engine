use crate::config::types::ProfileConfig;
use crate::error::{EngineError, Result};
use crate::optimizer::{OptimizationState, Optimizer};
use async_trait::async_trait;

#[derive(Debug, Clone, Default)]
pub struct CpuOptimizer;

#[async_trait]
impl Optimizer for CpuOptimizer {
    async fn apply(&mut self, profile: &ProfileConfig, pids: &[u32], state: &mut OptimizationState) -> Result<()> {
        if let Some(governor) = &profile.cpu_governor {
            let current = crate::system::SystemUtils::get_current_cpu_governor();
            if current.as_deref() != Some(governor) {
                crate::system::SystemUtils::set_cpu_governor(governor)
                    .map_err(|e| EngineError::Optimizer(format!("Cannot set CPU governor: {}", e)))?;
                state.cpu_governor = current;
                info!("CPU governor set to {}", governor);
            }
        }

        if let Some(affinity) = &profile.cpu_affinity {
            for pid in pids {
                let current = crate::system::SystemUtils::get_cpu_affinity(*pid);
                if current != *affinity {
                    crate::system::SystemUtils::set_cpu_affinity(*pid, affinity)
                        .map_err(|e| EngineError::Optimizer(format!("Cannot set CPU affinity: {}", e)))?;
                    state.cpu_affinity.insert(*pid, current);
                    debug!("Set CPU affinity for PID {} to {:?}", pid, affinity);
                }
            }
        }

        Ok(())
    }

    async fn restore(&self, state: &OptimizationState) -> Result<()> {
        if let Some(governor) = &state.cpu_governor {
            crate::system::SystemUtils::set_cpu_governor(governor)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore CPU governor: {}", e)))?;
            info!("CPU governor restored to {}", governor);
        }

        for (pid, affinity) in &state.cpu_affinity {
            crate::system::SystemUtils::set_cpu_affinity(*pid, affinity)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore CPU affinity: {}", e)))?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "cpu"
    }
}
