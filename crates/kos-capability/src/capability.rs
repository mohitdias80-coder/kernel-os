/// A service that can be executed by Kernel OS.
///
/// Examples:
/// - OCR
/// - Speech Recognition
/// - Translation
/// - LLM
/// - Vision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    name: String,
    description: String,
    version: String,
}

impl Capability {
    /// Create a new capability.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: version.into(),
        }
    }

    /// Capability name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Human-readable description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Capability version.
    pub fn version(&self) -> &str {
        &self.version
    }
}

