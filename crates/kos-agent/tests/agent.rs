use kos_agent::*;

#[test]
fn manager_starts_empty() {
    let manager = AgentManager::new();
    assert!(manager.is_empty());
}

#[test]
fn agent_can_be_registered() {
    let mut manager = AgentManager::new();

let agent = Agent::new(kos_core::AgentId(1), "Planner");
    let id = agent.id();

    manager.register(agent);

    assert_eq!(manager.len(), 1);
    assert!(manager.get(id).is_some());
}

#[test]
fn agent_can_be_removed() {
    let mut manager = AgentManager::new();

let agent = Agent::new(kos_core::AgentId(2), "Executor");
    let id = agent.id();

    manager.register(agent);
    manager.unregister(id);

    assert!(manager.is_empty());
}
