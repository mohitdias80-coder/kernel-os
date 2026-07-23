/// Represents the current execution state of an AI agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// Agent has been created but not started.
    Idle,

    /// Agent is planning its next actions.
    Planning,

    /// Agent is actively executing work.
    Running,

    /// Agent is waiting for an external event.
    Waiting,

    /// Agent has completed its mission.
    Completed,

    /// Agent encountered an unrecoverable error.
    Failed,
}

impl AgentState {
    /// Returns true if the agent is actively doing work.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Planning | Self::Running)
    }

    /// Returns true if execution has finished.
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

impl Default for AgentState {
    fn default() -> Self {
        Self::Idle
    }
}

