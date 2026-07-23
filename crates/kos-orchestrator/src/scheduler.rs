/// Kernel OS mission scheduler.
///
/// Responsible for advancing scheduling cycles and, in future,
/// deciding which missions and agents execute next.
#[derive(Debug, Default)]
pub struct Scheduler {
    tick_count: u64,
}

impl Scheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self {
            tick_count: 0,
        }
    }

    /// Advance the scheduler by one tick.
    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    /// Return the current scheduler tick.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Reset the scheduler.
    pub fn reset(&mut self) {
        self.tick_count = 0;
    }
}
