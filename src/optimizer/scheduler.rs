use crate::config::types::ProfileConfig;
use crate::error::EngineError;
use crate::optimizer::{OptimizationState, Optimizer};

#[derive(Debug, Clone, Default)]
pub struct SchedulerOptimizer;

#[async_trait::async_trait]
impl Optimizer for SchedulerOptimizer {
    async fn apply(
        &mut self,
        profile: &ProfileConfig,
        _pids: &[u32],
        state: &mut OptimizationState,
    ) -> crate::error::Result<()> {
        if let Some(scheduler) = &profile.scheduler {
            let current = Self::get_current_scheduler();
            if current.as_deref() != Some(scheduler) {
                Self::set_scheduler(scheduler)
                    .map_err(|e| EngineError::Optimizer(format!("Cannot set scheduler: {}", e)))?;
                state.scheduler = current;
                info!("Scheduler set to {}", scheduler);
            }
        }
        Ok(())
    }

    async fn restore(&self, state: &OptimizationState) -> crate::error::Result<()> {
        if let Some(scheduler) = &state.scheduler {
            Self::set_scheduler(scheduler)
                .map_err(|e| EngineError::Optimizer(format!("Cannot restore scheduler: {}", e)))?;
            info!("Scheduler restored to {}", scheduler);
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "scheduler"
    }
}

impl SchedulerOptimizer {
    fn get_current_scheduler() -> Option<String> {
        crate::system::SystemUtils::read_file("/sys/devices/system/cpu/sched_schedstat")
            .map(|_| "default".to_string())
    }

    fn set_scheduler(scheduler: &str) -> Result<(), String> {
        match scheduler {
            "default" | "normal" => {
                crate::system::SystemUtils::run_command("chrt", &["-d", "0", "0"]).map(|_| ())
            }
            "batch" => {
                crate::system::SystemUtils::run_command("chrt", &["-b", "0", "0"]).map(|_| ())
            }
            _ => Err(format!("Unknown scheduler: {}", scheduler)),
        }
    }
}
