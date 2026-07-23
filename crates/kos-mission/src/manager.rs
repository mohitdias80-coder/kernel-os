use std::collections::HashMap;

use kos_core::MissionId;

use crate::Mission;


#[derive(Debug, Default)]
pub struct MissionManager {
    missions: HashMap<MissionId, Mission>,
}


impl MissionManager {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
        }
    }


    pub fn register(&mut self, mission: Mission) {
        self.missions.insert(mission.id(), mission);
    }


    pub fn get(&self, id: MissionId) -> Option<&Mission> {
        self.missions.get(&id)
    }


    pub fn remove(&mut self, id: MissionId) -> Option<Mission> {
        self.missions.remove(&id)
    }


    pub fn count(&self) -> usize {
        self.missions.len()
    }
}
