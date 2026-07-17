use kos_core::MissionId;

use crate::{
    FifoScheduler,
    Mission,
    MissionManager,
    Scheduler,
};

/// Main Kernel OS Mission Runtime.
///
/// Responsible for:
/// - Creating missions
/// - Registering missions
/// - Scheduling execution
pub struct Runtime {
    manager: MissionManager,
    scheduler: FifoScheduler,
    next_id: u64,
}

impl Runtime {
    /// Create a new runtime.
    pub fn new() -> Self {
        Self {
            manager: MissionManager::new(),
            scheduler: FifoScheduler::new(),
            next_id: 1,
        }
    }

    /// Create a new Mission.
    pub fn create_mission(
        &mut self,
        name: &str,
    ) -> MissionId {

        let id = MissionId(self.next_id);

        self.next_id += 1;

        let mission = Mission::new(id, name);

        self.manager.insert(mission);

        self.scheduler.enqueue(id);

        id
    }

    /// Get immutable MissionManager.
    pub fn manager(&self) -> &MissionManager {
        &self.manager
    }

    /// Get mutable MissionManager.
    pub fn manager_mut(&mut self) -> &mut MissionManager {
        &mut self.manager
    }

    /// Access scheduler.
    pub fn scheduler(&mut self) -> &mut FifoScheduler {
        &mut self.scheduler
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

