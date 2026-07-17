use std::collections::VecDeque;

use kos_core::MissionId;

/// Scheduler interface.
pub trait Scheduler {
    /// Add a mission to the scheduling queue.
    fn enqueue(&mut self, id: MissionId);

    /// Return the next mission to execute.
    fn next(&mut self) -> Option<MissionId>;

    /// Number of queued missions.
    fn len(&self) -> usize;

    /// Returns true if no missions are queued.
    fn is_empty(&self) -> bool;
}

/// FIFO scheduler implementation.
pub struct FifoScheduler {
    queue: VecDeque<MissionId>,
}

impl FifoScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl Scheduler for FifoScheduler {
    fn enqueue(&mut self, id: MissionId) {
        self.queue.push_back(id);
    }

    fn next(&mut self) -> Option<MissionId> {
        self.queue.pop_front()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
