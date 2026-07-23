#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissionState {
    Created,
    Running,
    Paused,
    Completed,
    Failed,
}

impl Default for MissionState {
    fn default() -> Self {
        Self::Created
    }
}
