use crate::{BackendRegistry, ComputeBackend, ComputeDevice};

/// Public interface to the Compute Fabric.
pub struct ComputeManager {
    registry: BackendRegistry,
}

impl ComputeManager {
    /// Create a new Compute Manager.
    pub fn new() -> Self {
        Self {
            registry: BackendRegistry::new(),
        }
    }

    /// Register a compute backend.
    pub fn register_backend<B>(&mut self, backend: B)
    where
        B: ComputeBackend + 'static,
    {
        self.registry.register(backend);
    }

    /// Return names of all available backends.
    pub fn backend_names(&self) -> Vec<String> {
        self.registry.backend_names()
    }

    /// Collect all available compute devices.
    pub fn devices(&self) -> Vec<ComputeDevice> {
        let mut devices = Vec::new();

        for backend in self.registry.backend_names() {
            // Placeholder for MVP.
            // In later phases we'll query each backend directly.
            devices.push(ComputeDevice::new(
                0,
                backend,
                crate::DeviceKind::Cpu,
                0,
            ));
        }

        devices
    }

    /// Returns true if no backends are registered.
    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }
}

impl Default for ComputeManager {
    fn default() -> Self {
        Self::new()
    }
}
