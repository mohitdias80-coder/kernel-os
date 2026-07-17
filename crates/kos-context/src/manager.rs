use std::collections::HashMap;

use kos_core::ContextId;

use crate::ContextSpace;

/// Stores and manages all Context Spaces.
pub struct ContextManager {
    contexts: HashMap<ContextId, ContextSpace>,
}

impl ContextManager {
    /// Create a new Context Manager.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    /// Insert a Context Space.
    pub fn insert(&mut self, context: ContextSpace) {
        self.contexts.insert(context.id(), context);
    }

    /// Get immutable Context Space.
    pub fn get(&self, id: ContextId) -> Option<&ContextSpace> {
        self.contexts.get(&id)
    }

    /// Get mutable Context Space.
    pub fn get_mut(&mut self, id: ContextId) -> Option<&mut ContextSpace> {
        self.contexts.get_mut(&id)
    }

    /// Remove a Context Space.
    pub fn remove(&mut self, id: ContextId) -> Option<ContextSpace> {
        self.contexts.remove(&id)
    }

    /// Number of Context Spaces.
    pub fn len(&self) -> usize {
        self.contexts.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

