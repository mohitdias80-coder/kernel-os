/// Represents a simple execution plan for an agent.
#[derive(Debug, Clone)]
pub struct Plan {
    steps: Vec<String>,
}

impl Plan {
    /// Create an empty plan.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
        }
    }

    /// Add a step to the plan.
    pub fn add_step(&mut self, step: impl Into<String>) {
        self.steps.push(step.into());
    }

    /// Return all plan steps.
    pub fn steps(&self) -> &[String] {
        &self.steps
    }

    /// Returns true if the plan contains no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// Responsible for generating execution plans.
pub struct Planner;

impl Planner {
    /// Create a new planner.
    pub fn new() -> Self {
        Self
    }

    /// Build a simple plan from a goal.
    pub fn create_plan(&self, goal: impl Into<String>) -> Plan {
        let mut plan = Plan::new();
        plan.add_step(goal);
        plan
    }
}

