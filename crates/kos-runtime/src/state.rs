use serde::{Deserialize, Serialize};

/// Lifecycle states of a Mission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionState {
    /// Mission has been created but not yet scheduled.
    Created,

    /// Mission is waiting in the scheduler queue.
    Queued,

    /// Mission is currently executing.
    Running,

    /// Mission is waiting for an external event.
    Waiting,

    /// Mission has been temporarily paused.
    Suspended,

    /// Mission is migrating to another runtime or node.
    Migrating,

    /// Mission finished successfully.
    Completed,

    /// Mission terminated with an error.
    Failed,

    /// Mission is retained for history/audit only.
    Archived,
}

