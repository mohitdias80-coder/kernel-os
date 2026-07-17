use crate::ComputeBackend;

/// Stores all registered compute backends.
pub struct BackendRegistry {
    backends: Vec<Box<dyn ComputeBackend>>,
}

impl BackendRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            backends: Vec::new(),
        }
    }

    /// Register a backend.
    pub fn register<B>(&mut self, backend: B)
    where
        B: ComputeBackend + 'static,
    {
        self.backends.push(Box::new(backend));
    }

    /// Number of registered backends.
    pub fn len(&self) -> usize {
        self.backends.len()
    }

    /// Returns true if no backends are registered.
    pub fn is_empty(&self) -> bool {
        self.backends.is_empty()
    }

    /// Returns the names of available backends.
    pub fn backend_names(&self) -> Vec<String> {
        self.backends
            .iter()
            .filter(|b| b.available())
            .map(|b| b.name().to_string())
            .collect()
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}

