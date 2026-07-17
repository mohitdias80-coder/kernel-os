use kos_core::ContextId;

use crate::{Document, KnowledgeStore};

/// High-level manager for the Knowledge Fabric.
#[derive(Debug, Default)]
pub struct KnowledgeManager {
    store: KnowledgeStore,
}

impl KnowledgeManager {
    /// Create an empty manager.
    pub fn new() -> Self {
        Self {
            store: KnowledgeStore::new(),
        }
    }

    /// Access the underlying store.
    pub fn store(&self) -> &KnowledgeStore {
        &self.store
    }

    /// Access the underlying store mutably.
    pub fn store_mut(&mut self) -> &mut KnowledgeStore {
        &mut self.store
    }

    /// Add a document.
    pub fn add_document(&mut self, document: Document) {
        self.store.insert(document);
    }

    /// Get a document.
    pub fn get_document(&self, id: ContextId) -> Option<&Document> {
        self.store.get(id)
    }

    /// Remove a document.
    pub fn remove_document(&mut self, id: ContextId) -> Option<Document> {
        self.store.remove(id)
    }
}

