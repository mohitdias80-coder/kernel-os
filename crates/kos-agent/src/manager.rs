use std::collections::HashMap;

use kos_core::AgentId;

use crate::Agent;

/// Manages all agents registered in the Kernel OS runtime.
#[derive(Debug, Default)]
pub struct AgentManager {
    agents: HashMap<AgentId, Agent>,
}

impl AgentManager {
    /// Creates an empty Agent Manager.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Registers a new agent.
    pub fn register(&mut self, agent: Agent) {
        self.agents.insert(agent.id(), agent);
    }

    /// Removes an agent.
    pub fn unregister(&mut self, id: AgentId) -> Option<Agent> {
        self.agents.remove(&id)
    }

    /// Returns an immutable reference to an agent.
    pub fn get(&self, id: AgentId) -> Option<&Agent> {
        self.agents.get(&id)
    }

    /// Returns a mutable reference to an agent.
    pub fn get_mut(&mut self, id: AgentId) -> Option<&mut Agent> {
        self.agents.get_mut(&id)
    }

    /// Returns all registered agents.
    pub fn agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }

    /// Number of registered agents.
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Returns true if no agents are registered.
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Removes all agents.
    pub fn clear(&mut self) {
        self.agents.clear();
    }
}
