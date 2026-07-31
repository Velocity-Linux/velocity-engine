pub mod cpu;
pub mod io;
pub mod power;
pub mod scheduler;

use std::collections::HashMap;

use crate::config::types::ProfileConfig;
use crate::error::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, Default)]
pub struct OptimizationState {
    pub cpu_governor: Option<String>,
    pub cpu_affinity: HashMap<u32, Vec<usize>>,
    pub process_priorities: HashMap<u32, i32>,
    pub io_priorities: HashMap<u32, (String, i32)>,
    pub power_profile: Option<String>,
    pub scheduler: Option<String>,
}

#[async_trait]
pub trait Optimizer: Send + Sync {
    async fn apply(
        &mut self,
        profile: &ProfileConfig,
        pids: &[u32],
        state: &mut OptimizationState,
    ) -> Result<()>;
    async fn restore(&self, state: &OptimizationState) -> Result<()>;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct OptimizerManager {
    cpu: cpu::CpuOptimizer,
    io: io::IoOptimizer,
    power: power::PowerOptimizer,
    scheduler: scheduler::SchedulerOptimizer,
}

impl OptimizerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn apply_profile(
        &mut self,
        profile: &ProfileConfig,
        pids: &[u32],
    ) -> Result<()> {
        let mut state = OptimizationState::default();
        self.cpu.apply(profile, pids, &mut state).await?;
        self.io.apply(profile, pids, &mut state).await?;
        self.power.apply(profile, pids, &mut state).await?;
        self.scheduler.apply(profile, pids, &mut state).await?;
        Ok(())
    }

    pub async fn restore(&self) -> Result<()> {
        let state = OptimizationState::default();
        self.cpu.restore(&state).await?;
        self.io.restore(&state).await?;
        self.power.restore(&state).await?;
        self.scheduler.restore(&state).await?;
        Ok(())
    }

    pub fn list_optimizers(&self) -> Vec<String> {
        vec![
            self.cpu.name().to_string(),
            self.io.name().to_string(),
            self.power.name().to_string(),
            self.scheduler.name().to_string(),
        ]
    }
}
