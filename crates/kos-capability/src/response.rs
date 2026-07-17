/// Result returned by a Capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityResponse {
    success: bool,
    output: String,
}

impl CapabilityResponse {
    /// Create a successful response.
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            success: true,
            output: output.into(),
        }
    }

    /// Create a failed response.
    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            success: false,
            output: output.into(),
        }
    }

    /// Whether the capability succeeded.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Response payload.
    pub fn output(&self) -> &str {
        &self.output
    }
}

