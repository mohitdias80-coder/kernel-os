use thiserror::Error;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Invalid mission")]
    InvalidMission,

    #[error("Invalid context")]
    InvalidContext,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Capability unavailable")]
    CapabilityUnavailable,

    #[error("Compute unavailable")]
    ComputeUnavailable,

    #[error("Internal error")]
    InternalError,
}

pub type KernelResult<T> = Result<T, KernelError>;
