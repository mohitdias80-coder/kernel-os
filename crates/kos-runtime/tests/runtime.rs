use kos_runtime::Runtime;

#[test]
fn runtime_can_be_created() {
    let runtime = Runtime::new();

    assert!(runtime.manager().is_empty());
}

#[test]
fn mission_creation_works() {
    let mut runtime = Runtime::new();

    let id = runtime.create_mission("Test Mission");

    assert_eq!(runtime.manager().len(), 1);
    assert!(runtime.manager().get(id).is_some());
}

#[test]
fn mission_ids_are_unique() {
    let mut runtime = Runtime::new();

    let id1 = runtime.create_mission("Mission A");
    let id2 = runtime.create_mission("Mission B");

    assert_ne!(id1, id2);
}

