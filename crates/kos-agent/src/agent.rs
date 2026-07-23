use kos_core::{AgentId, MissionId};

/// Represents an AI Agent managed by Kernel OS.
#[derive(Debug, Clone)]
pub struct Agent {
    id: AgentId,
    name: String,
    mission: Option<MissionId>,
    active: bool,
}

impl Agent {
    /// Create a new agent.
    pub fn new(id: AgentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            mission: None,
            active: false,
        }
    }

    /// Returns the agent ID.
    pub fn id(&self) -> AgentId {
        self.id
    }

    /// Returns the agent name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the assigned mission.
    pub fn mission(&self) -> Option<MissionId> {
        self.mission
    }

    /// Assign a mission.
    pub fn assign_mission(&mut self, mission: MissionId) {
        self.mission = Some(mission);
    }

    /// Check whether the agent is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activate the agent.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate the agent.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

