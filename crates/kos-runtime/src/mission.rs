use kos_core::{ContextId, MissionId};

use crate::MissionState;

/// Represents a Mission managed by the Kernel OS Runtime.
///
/// Every Mission owns exactly one Context.
#[derive(Debug, Clone)]
pub struct Mission {
    id: MissionId,
    context: ContextId,
    name: String,
    state: MissionState,
}

impl Mission {
    /// Create a new Mission.
    pub fn new(
        id: MissionId,
        context: ContextId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            context,
            name: name.into(),
            state: MissionState::Created,
        }
    }

    /// Mission identifier.
    pub fn id(&self) -> MissionId {
        self.id
    }

    /// Context owned by this Mission.
    pub fn context(&self) -> ContextId {
        self.context
    }

    /// Mission name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Current Mission state.
    pub fn state(&self) -> MissionState {
        self.state
    }

    /// Update Mission state.
    pub fn set_state(&mut self, state: MissionState) {
        self.state = state;
    }
}
