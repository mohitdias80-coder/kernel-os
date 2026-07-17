use crate::Document;

/// Working memory for the Knowledge Fabric.
#[derive(Debug, Clone, Default)]
pub struct Memory {
    documents: Vec<Document>,
}

impl Memory {
    /// Create an empty memory.
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    /// Add a document.
    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
    }

    /// Number of stored documents.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Returns true if memory is empty.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Get all documents.
    pub fn documents(&self) -> &[Document] {
        &self.documents
    }
}

