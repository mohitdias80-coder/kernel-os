use std::fmt;

/// Type of compute device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    Cpu,
    Gpu,
    Npu,
    Tpu,
    Other,
}

/// Generic compute device description.
#[derive(Debug, Clone)]
pub struct ComputeDevice {
    id: u32,
    name: String,
    kind: DeviceKind,
    memory_mb: u64,
}

impl ComputeDevice {
    pub fn new(
        id: u32,
        name: impl Into<String>,
        kind: DeviceKind,
        memory_mb: u64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind,
            memory_mb,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> DeviceKind {
        self.kind
    }

    pub fn memory_mb(&self) -> u64 {
        self.memory_mb
    }
}

impl fmt::Display for ComputeDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:?}, {} MB)",
            self.name,
            self.kind,
            self.memory_mb
        )
    }
}
