use kos_compute::{
    BackendRegistry,
    ComputeManager,
    CpuBackend,
};

#[test]
fn registry_starts_empty() {
    let registry = BackendRegistry::new();

    assert!(registry.is_empty());
}

#[test]
fn cpu_backend_can_be_registered() {
    let mut registry = BackendRegistry::new();

    registry.register(CpuBackend::new());

    assert_eq!(registry.len(), 1);
}

#[test]
fn manager_lists_backends() {
    let mut manager = ComputeManager::new();

    manager.register_backend(CpuBackend::new());

    let names = manager.backend_names();

    assert_eq!(names.len(), 1);
    assert_eq!(names[0], "CPU");
}

