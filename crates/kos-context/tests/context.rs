use kos_context::{ContextManager, ContextSpace};
use kos_core::ContextId;

#[test]
fn context_manager_starts_empty() {
    let manager = ContextManager::new();

    assert!(manager.is_empty());
}

#[test]
fn context_can_be_inserted() {
    let mut manager = ContextManager::new();

    let id = ContextId(1);

    let context = ContextSpace::new(id, "Main Context");

    manager.insert(context);

    assert_eq!(manager.len(), 1);

    assert!(manager.get(id).is_some());
}

#[test]
fn active_context_can_be_updated() {
    let id = ContextId(1);

    let mut context = ContextSpace::new(id, "Test");

    context.set_active_context("Hello Kernel OS");

    assert_eq!(context.active_context(), "Hello Kernel OS");
}

