use kos_core::ContextId;
use kos_knowledge::{Document, KnowledgeManager};

#[test]
fn manager_starts_empty() {
    let manager = KnowledgeManager::new();

    assert!(manager.store().is_empty());
}

#[test]
fn document_can_be_added() {
    let mut manager = KnowledgeManager::new();

let id = ContextId(1);

    manager.add_document(Document::new(
        id,
        "README",
        "Kernel OS documentation",
    ));

    assert_eq!(manager.store().len(), 1);
    assert!(manager.get_document(id).is_some());
}

#[test]
fn document_can_be_removed() {
    let mut manager = KnowledgeManager::new();

let id = ContextId(2);

    manager.add_document(Document::new(
        id,
        "Notes",
        "Temporary document",
    ));

    assert!(manager.remove_document(id).is_some());
    assert!(manager.store().is_empty());
}

