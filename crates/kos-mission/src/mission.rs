use kos_core::MissionId;

use crate::MissionState;

#[derive(Debug, Clone)]
pub struct Mission {
    id: MissionId,
    name: String,
    state: MissionState,
}

impl Mission {
    pub fn new(id: MissionId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            state: MissionState::Created,
        }
    }

    pub fn id(&self) -> MissionId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &MissionState {
        &self.state
    }

    pub fn set_state(&mut self, state: MissionState) {
        self.state = state;
    }
}
