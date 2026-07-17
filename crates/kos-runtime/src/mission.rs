use kos_core::MissionId;

use crate::MissionState;

/// A Mission is the primary execution unit in Kernel OS.
/// Similar to a process in Linux, but specialized for AI workloads.
#[derive(Debug, Clone)]
pub struct Mission {
    id: MissionId,
    name: String,
    state: MissionState,
}

impl Mission {
    /// Create a new mission.
    pub fn new(id: MissionId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            state: MissionState::Created,
        }
    }

    /// Mission identifier.
    pub fn id(&self) -> MissionId {
        self.id
    }

    /// Human-readable mission name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Current lifecycle state.
    pub fn state(&self) -> MissionState {
        self.state
    }

    /// Transition to Queued.
    pub fn queue(&mut self) {
        self.state = MissionState::Queued;
    }

    /// Transition to Running.
    pub fn execute(&mut self) {
        self.state = MissionState::Running;
    }

    /// Transition to Waiting.
    pub fn wait(&mut self) {
        self.state = MissionState::Waiting;
    }

    /// Transition to Suspended.
    pub fn suspend(&mut self) {
        self.state = MissionState::Suspended;
    }

    /// Transition to Migrating.
    pub fn migrate(&mut self) {
        self.state = MissionState::Migrating;
    }

    /// Transition to Completed.
    pub fn complete(&mut self) {
        self.state = MissionState::Completed;
    }

    /// Transition to Failed.
    pub fn fail(&mut self) {
        self.state = MissionState::Failed;
    }

    /// Transition to Archived.
    pub fn archive(&mut self) {
        self.state = MissionState::Archived;
    }
}

