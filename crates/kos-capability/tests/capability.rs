
use kos_capability::{
    Capability,
    CapabilityManager,
};

#[test]
fn manager_starts_empty() {
    let manager = CapabilityManager::new();

    assert!(manager.is_empty());
}

#[test]
fn capability_can_be_registered() {
    let mut manager = CapabilityManager::new();

    manager.register(
        Capability::new(
            "ocr",
            "Optical Character Recognition",
            "1.0.0",
        ),
    );

    assert_eq!(manager.len(), 1);
}

#[test]
fn capability_lookup_works() {
    let mut manager = CapabilityManager::new();

    manager.register(
        Capability::new(
            "llm",
            "Large Language Model",
            "1.0.0",
        ),
    );

    assert!(manager.get("llm").is_some());
}
