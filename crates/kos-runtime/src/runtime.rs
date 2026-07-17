use kos_compute::ComputeManager;
use kos_context::{ContextManager, ContextSpace};
use kos_core::{ContextId, MissionId};

use crate::{Mission, MissionManager};

/// Central Kernel OS runtime.
pub struct Runtime {
    missions: MissionManager,
    contexts: ContextManager,
    compute: ComputeManager,

    next_mission_id: u64,
    next_context_id: u64,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            missions: MissionManager::new(),
            contexts: ContextManager::new(),
            compute: ComputeManager::new(),
            next_mission_id: 1,
            next_context_id: 1,
        }
    }

    /// Create a Mission and its Context together.
    pub fn create_mission(
        &mut self,
        name: impl Into<String>,
    ) -> MissionId {
        let mission_id = MissionId(self.next_mission_id);
        self.next_mission_id += 1;

        let context_id = ContextId(self.next_context_id);
        self.next_context_id += 1;

let context =
    ContextSpace::new(context_id, "Default");

        self.contexts.insert(context);

        let mission =
            Mission::new(mission_id, context_id, name);

        self.missions.insert(mission);

        mission_id
    }

    pub fn missions(&self) -> &MissionManager {
        &self.missions
    }

    pub fn contexts(&self) -> &ContextManager {
        &self.contexts
    }

    pub fn compute(&self) -> &ComputeManager {
        &self.compute
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

