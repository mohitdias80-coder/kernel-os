use crate::{
    Capability,
    CapabilityRegistry,
};

/// Manages registered capabilities.
#[derive(Debug, Default)]
pub struct CapabilityManager {
    registry: CapabilityRegistry,
}

impl CapabilityManager {
    /// Create a new manager.
    pub fn new() -> Self {
        Self {
            registry: CapabilityRegistry::new(),
        }
    }

    /// Register a capability.
    pub fn register(&mut self, capability: Capability) {
        self.registry.register(capability);
    }

    /// Lookup a capability.
    pub fn get(&self, name: &str) -> Option<&Capability> {
        self.registry.get(name)
    }

    /// Remove a capability.
    pub fn remove(&mut self, name: &str) -> Option<Capability> {
        self.registry.remove(name)
    }

    /// Number of capabilities.
    pub fn len(&self) -> usize {
        self.registry.len()
    }

    /// Whether the manager is empty.
    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }
}

