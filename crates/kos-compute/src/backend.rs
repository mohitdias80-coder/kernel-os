use crate::ComputeDevice;

/// Every compute backend (CPU, CUDA, ROCm, Metal, etc.)
/// implements this interface.
pub trait ComputeBackend {
    /// Human-readable backend name.
    fn name(&self) -> &str;

    /// Devices exposed by this backend.
    fn devices(&self) -> Vec<ComputeDevice>;

    /// Returns true if the backend is available.
    fn available(&self) -> bool;
}

/// Simple CPU backend for the MVP.
pub struct CpuBackend;

impl CpuBackend {
    pub fn new() -> Self {
        Self
    }
}

impl ComputeBackend for CpuBackend {
    fn name(&self) -> &str {
        "CPU"
    }

    fn devices(&self) -> Vec<ComputeDevice> {
        vec![
            ComputeDevice::new(
                0,
                "Generic CPU",
                crate::DeviceKind::Cpu,
                8192,
            )
        ]
    }

    fn available(&self) -> bool {
        true
    }
}

