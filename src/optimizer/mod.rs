use std::collections::HashMap;

use crate::config::types::ProfileConfig;
use crate::error::Result;
use crate::optimizer::cpu::CpuOptimizer;
use crate::optimizer::io::IoOptimizer;
use crate::optimizer::power::PowerOptimizer;
use crate::optimizer::scheduler::SchedulerOptimizer;
use crate::optimizer::{OptimizationState, Optimizer};
use async_trait::async_trait;

pub struct OptimizerManager {
    optimizers: Vec<Box<dyn Optimizer>>,
    current_state: OptimizationState,
}

impl OptimizerManager {
    pub fn new() -> Self {
        Self {
            optimizers: vec![
                Box::new(CpuOptimizer),
                Box::new(PowerOptimizer),
                Box::new(IoOptimizer),
                Box::new(SchedulerOptimizer),
            ],
            current_state: OptimizationState::default(),
        }
    }

    pub async fn apply_profile(&mut self, profile: &ProfileConfig, pids: &[u32]) -> Result<()> {
        for optimizer in &mut self.optimizers {
            optimizer
                .apply(profile, pids, &mut self.current_state)
                .await?;
        }
        Ok(())
    }

    pub async fn restore(&self) -> Result<()> {
        for optimizer in &self.optimizers {
            optimizer.restore(&self.current_state).await?;
        }
        Ok(())
    }

    pub fn list_optimizers(&self) -> Vec<String> {
        self.optimizers
            .iter()
            .map(|o| o.name().to_string())
            .collect()
    }
}
