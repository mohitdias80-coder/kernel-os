use crate::scheduler::Scheduler;

/// The central Kernel OS orchestrator.
///
/// The orchestrator is responsible for coordinating all
/// Kernel OS fabrics and scheduling mission execution.
#[derive(Debug)]
pub struct Orchestrator {
    scheduler: Scheduler,
}

impl Orchestrator {
    /// Create a new orchestrator.
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
        }
    }

    /// Immutable access to the scheduler.
    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// Mutable access to the scheduler.
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    /// Advance the orchestrator by one scheduling cycle.
    ///
    /// Future versions will:
    /// - Poll runtime events
    /// - Dispatch capabilities
    /// - Coordinate agents
    /// - Manage compute resources
    /// - Update knowledge contexts
    pub fn tick(&mut self) {
        self.scheduler.tick();
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}
