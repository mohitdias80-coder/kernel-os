/// A request sent to a Capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequest {
    capability: String,
    input: String,
}

impl CapabilityRequest {
    /// Create a new capability request.
    pub fn new(
        capability: impl Into<String>,
        input: impl Into<String>,
    ) -> Self {
        Self {
            capability: capability.into(),
            input: input.into(),
        }
    }

    /// Requested capability name.
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Input payload.
    pub fn input(&self) -> &str {
        &self.input
    }
}

