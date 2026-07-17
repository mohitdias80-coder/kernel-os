use std::collections::HashMap;

use crate::Capability;

/// Stores all registered capabilities.
#[derive(Debug, Default)]
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capability>,
}

impl CapabilityRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }

    /// Register a capability.
    pub fn register(&mut self, capability: Capability) {
        self.capabilities
            .insert(capability.name().to_string(), capability);
    }

    /// Get a capability by name.
    pub fn get(&self, name: &str) -> Option<&Capability> {
        self.capabilities.get(name)
    }

    /// Remove a capability.
    pub fn remove(&mut self, name: &str) -> Option<Capability> {
        self.capabilities.remove(name)
    }

    /// Number of registered capabilities.
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
}

