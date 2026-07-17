use crate::Permission;

/// Defines a collection of permissions.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    name: String,
    permissions: Vec<Permission>,
}

impl Policy {
    /// Create a new policy.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            permissions: Vec::new(),
        }
    }

    /// Policy name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add a permission.
    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.push(permission);
    }

    /// Get all permissions.
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Check whether a permission exists.
    pub fn has_permission(&self, name: &str) -> bool {
        self.permissions
            .iter()
            .any(|permission| permission.name() == name)
    }
}

