use std::collections::HashMap;

use kos_core::ContextId;

use crate::Document;

/// Stores all knowledge documents.
#[derive(Debug, Default)]
pub struct KnowledgeStore {
    documents: HashMap<ContextId, Document>,
}

impl KnowledgeStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Insert a document.
    pub fn insert(&mut self, document: Document) {
        self.documents.insert(document.id(), document);
    }

    /// Get a document.
    pub fn get(&self, id: ContextId) -> Option<&Document> {
        self.documents.get(&id)
    }

    /// Remove a document.
    pub fn remove(&mut self, id: ContextId) -> Option<Document> {
        self.documents.remove(&id)
    }

    /// Number of stored documents.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

