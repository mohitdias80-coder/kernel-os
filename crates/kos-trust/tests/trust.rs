use kos_trust::{
    Permission,
    Policy,
    TrustManager,
};

#[test]
fn manager_starts_with_default_policy() {
    let manager = TrustManager::new();

    assert_eq!(manager.policy().name(), "default");
}

#[test]
fn permission_can_be_granted() {
    let mut policy = Policy::new("admin");

    policy.add_permission(
        Permission::new(
            "compute.execute",
            "Allow compute execution",
        ),
    );

    assert!(policy.has_permission("compute.execute"));
}

#[test]
fn trust_manager_checks_permission() {
    let mut manager = TrustManager::new();

    let mut policy = Policy::new("runtime");

    policy.add_permission(
        Permission::new(
            "llm.generate",
            "Allow LLM generation",
        ),
    );

    manager.set_policy(policy);

    assert!(manager.is_allowed("llm.generate"));
    assert!(!manager.is_allowed("filesystem.write"));
}

