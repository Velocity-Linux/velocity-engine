use crate::config::types::ProfileConfig;
use crate::error::EngineError;
use crate::optimizer::{OptimizationState, Optimizer};
use crate::system::SystemUtils;
use tracing::{debug, warn};

#[derive(Debug, Clone, Default)]
pub struct IoOptimizer;

#[async_trait::async_trait]
impl Optimizer for IoOptimizer {
    async fn apply(
        &mut self,
        profile: &ProfileConfig,
        pids: &[u32],
        state: &mut OptimizationState,
    ) -> crate::error::Result<()> {
        if let (Some(io_class), Some(priority)) = (&profile.io_priority, profile.process_priority) {
            for pid in pids {
                let current_priority = unsafe { libc::getpriority(libc::PRIO_PROCESS, *pid) };
                let current_priority = if current_priority == -1
                    && std::io::Error::last_os_error().raw_os_error() == Some(0)
                {
                    0
                } else {
                    current_priority
                };

                if current_priority != priority {
                    SystemUtils::set_process_priority(*pid, priority).map_err(|e| {
                        EngineError::Optimizer(format!("Cannot set process priority: {}", e))
                    })?;
                    state.process_priorities.insert(*pid, current_priority);
                    debug!("Set priority for PID {} to {}", pid, priority);
                }

                SystemUtils::set_io_priority(*pid, io_class, priority).map_err(|e| {
                    EngineError::Optimizer(format!("Cannot set I/O priority: {}", e))
                })?;
                state
                    .io_priorities
                    .insert(*pid, (io_class.clone(), priority));
                debug!("Set I/O priority for PID {} to {}", pid, io_class);
            }
        }

        Ok(())
    }

    async fn restore(&self, state: &OptimizationState) -> crate::error::Result<()> {
        for (pid, priority) in &state.process_priorities {
            if let Err(e) = SystemUtils::set_process_priority(*pid, *priority) {
                warn!("Cannot restore priority for PID {}: {}", pid, e);
            }
        }

        for (pid, (class, priority)) in &state.io_priorities {
            if let Err(e) = SystemUtils::set_io_priority(*pid, class, *priority) {
                warn!("Cannot restore I/O priority for PID {}: {}", pid, e);
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "io"
    }
}
