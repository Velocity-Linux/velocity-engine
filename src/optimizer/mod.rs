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
