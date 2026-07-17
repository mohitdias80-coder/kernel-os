use std::collections::HashMap;

use kos_core::MissionId;

use crate::Mission;

/// Stores and manages all Missions in the runtime.
pub struct MissionManager {
    missions: HashMap<MissionId, Mission>,
}

impl MissionManager {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, mission: Mission) {
        self.missions.insert(mission.id(), mission);
    }

    pub fn get(&self, id: MissionId) -> Option<&Mission> {
        self.missions.get(&id)
    }

    pub fn get_mut(&mut self, id: MissionId) -> Option<&mut Mission> {
        self.missions.get_mut(&id)
    }

    pub fn remove(&mut self, id: MissionId) -> Option<Mission> {
        self.missions.remove(&id)
    }

    pub fn len(&self) -> usize {
        self.missions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.missions.is_empty()
    }
}

