use kos_runtime::Runtime;

#[test]
fn runtime_can_be_created() {
    let runtime = Runtime::new();

    assert!(runtime.missions().is_empty());
    assert!(runtime.contexts().is_empty());
}

#[test]
fn runtime_creates_mission() {
    let mut runtime = Runtime::new();

    let id = runtime.create_mission("Demo Mission");

    assert!(runtime.missions().get(id).is_some());
}

#[test]
fn runtime_creates_context() {
    let mut runtime = Runtime::new();

    runtime.create_mission("Demo");

    assert_eq!(runtime.contexts().len(), 1);
}

