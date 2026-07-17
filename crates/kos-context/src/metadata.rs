use std::time::SystemTime;

/// Metadata describing a Context Space.
#[derive(Debug, Clone)]
pub struct ContextMetadata {
    /// Human-readable context name.
    pub name: String,

    /// Creation timestamp.
    pub created_at: SystemTime,

    /// Last update timestamp.
    pub updated_at: SystemTime,

    /// Context version.
    pub version: u64,
}

impl ContextMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        let now = SystemTime::now();

        Self {
            name: name.into(),
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = SystemTime::now();
        self.version += 1;
    }
}

