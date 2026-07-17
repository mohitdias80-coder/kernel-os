use kos_core::ContextId;

/// A document stored in the Knowledge Fabric.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    id: ContextId,
    title: String,
    content: String,
}

impl Document {
    /// Create a new document.
    pub fn new(
        id: ContextId,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
        }
    }

    /// Document identifier.
    pub fn id(&self) -> ContextId {
        self.id
    }

    /// Document title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Document content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Replace the document content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }
}

