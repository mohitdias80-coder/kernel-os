use kos_core::MissionId;
use kos_mission::{Mission, MissionManager};


#[test]
fn manager_starts_empty() {
    let manager = MissionManager::new();

    assert_eq!(manager.count(), 0);
}


#[test]
fn mission_can_be_registered() {
    let mut manager = MissionManager::new();

    let mission = Mission::new(MissionId::new(1), "Test Mission");

    manager.register(mission);

    assert_eq!(manager.count(), 1);
}


#[test]
fn mission_can_be_removed() {
    let mut manager = MissionManager::new();

    let mission = Mission::new(MissionId::new(1), "Remove Test");

    manager.register(mission);

    let removed = manager.remove(MissionId::new(1));

    assert!(removed.is_some());
    assert_eq!(manager.count(), 0);
}
