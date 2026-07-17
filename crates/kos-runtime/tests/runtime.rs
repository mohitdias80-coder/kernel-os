use kos_runtime::Runtime;

#[test]
fn runtime_can_be_created() {
    let runtime = Runtime::new();

    assert!(runtime.missions().is_empty());
    assert!(runtime.contexts().is_empty());
}

#[test]
fn mission_creation_works() {
    let mut runtime = Runtime::new();

    let id = runtime.create_mission("First Mission");

    assert_eq!(runtime.missions().len(), 1);
    assert!(runtime.missions().get(id).is_some());
}

#[test]
fn mission_has_context() {
    let mut runtime = Runtime::new();

    runtime.create_mission("Mission");

    assert_eq!(runtime.contexts().len(), 1);
}

