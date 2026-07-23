/// Result of executing a plan step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    success: bool,
    message: String,
}

impl ExecutionResult {
    /// Create a successful execution result.
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
        }
    }

    /// Create a failed execution result.
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
        }
    }

    /// Returns whether execution succeeded.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Returns the execution message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Executes agent plans.
///
/// This is a placeholder implementation for the MVP.
/// Future versions will dispatch Capability requests,
/// schedule Compute backends, and coordinate Runtime execution.
pub struct Executor;

impl Executor {
    /// Create a new executor.
    pub fn new() -> Self {
        Self
    }

    /// Execute a single plan step.
    pub fn execute(&self, step: impl Into<String>) -> ExecutionResult {
        let step = step.into();

        ExecutionResult::success(format!("Executed step: {}", step))
    }
}
