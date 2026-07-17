/// A permission that may be granted to an identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    name: String,
    description: String,
}

impl Permission {
    /// Create a new permission.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    /// Permission name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Human-readable description.
    pub fn description(&self) -> &str {
        &self.description
    }
}
