/// Represents the identity of a caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    name: String,
    role: String,
}

impl Identity {
    /// Create a new identity.
    pub fn new(
        name: impl Into<String>,
        role: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            role: role.into(),
        }
    }

    /// Identity name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Identity role.
    pub fn role(&self) -> &str {
        &self.role
    }
}

