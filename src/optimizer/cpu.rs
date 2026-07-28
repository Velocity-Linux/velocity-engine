use crate::config::types::ProfileConfig;
use crate::error::EngineError;
use crate::optimizer::{OptimizationState, Optimizer};
use crate::system::SystemUtils;

#[derive(Debug, Clone, Default)]
pub struct CpuOptimizer;

impl CpuOptimizer {
    async fn apply_impl(
        &mut self,
        profile: &ProfileConfig,
        pids: &[u32],
        state: &mut OptimizationState,
    ) -> crate::error::Result<()> {
        if let Some(governor) = &profile.cpu_governor {
            let current = SystemUtils::get_current_cpu_governor();
            if current.as_deref() != Some(governor) {
                SystemUtils::set_cpu_governor(governor)
                    .map_err(|e| EngineError::Optimizer(format!("Cannot set CPU governor: {}", e)))?;
                state.cpu_governor = current;
                info!("CPU governor set to {}", governor);
            }
        }

        if let Some(affinity) = &profile.cpu_affinity {
            for pid in pids {
                let current = SystemUtils::get_cpu_affinity(*pid);
                if current != *affinity {
                    SystemUtils::set_cpu_affinity(*pid, affinity)
                        .map_err(|e| EngineError::Optimizer(format!("Cannot set CPU affinity: {}", e)))?;
                    state.cpu_affinity.insert(*pid, current);
                    debug!("Set CPU affinity for PID {} to {:?}", pid, affinity);
                }
            }
        }

        Ok(())
    }

    async fn restore_impl(&self, state: &OptimizationState) -> crate::error::Result<()> {
        if let Some(governor) = &state.cpu_governor {
            SystemUtils::set_cpu_governor(governor)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore CPU governor: {}", e)))?;
            info!("CPU governor restored to {}", governor);
        }

        for (pid, affinity) in &state.cpu_affinity {
            SystemUtils::set_cpu_affinity(*pid, affinity)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore CPU affinity: {}", e)))?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Optimizer for CpuOptimizer {
    async fn apply(
        &mut self,
        profile: &ProfileConfig,
        pids: &[u32],
        state: &mut OptimizationState,
    ) -> crate::error::Result<()> {
        self.apply_impl(profile, pids, state).await
    }

    async fn restore(&self, state: &OptimizationState) -> crate::error::Result<()> {
        self.restore_impl(state).await
    }

    fn name(&self) -> &'static str {
        "cpu"
    }
}
