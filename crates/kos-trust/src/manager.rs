use crate::Policy;

/// Manages trust policies and authorization checks.
#[derive(Debug, Default)]
pub struct TrustManager {
    policy: Policy,
}

impl TrustManager {
    /// Create a new TrustManager with a default policy.
    pub fn new() -> Self {
        Self {
            policy: Policy::new("default"),
        }
    }

    /// Replace the active policy.
    pub fn set_policy(&mut self, policy: Policy) {
        self.policy = policy;
    }

    /// Get the active policy.
    pub fn policy(&self) -> &Policy {
        &self.policy
    }

    /// Check whether a permission is allowed.
    pub fn is_allowed(&self, permission: &str) -> bool {
        self.policy.has_permission(permission)
    }
}

