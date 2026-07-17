use kos_core::*;

#[test]
fn mission_id_equality() {
    let a = MissionId(1);
    let b = MissionId(1);

    assert_eq!(a, b);
}

#[test]
fn context_id_equality() {
    let a = ContextId(10);
    let b = ContextId(10);

    assert_eq!(a, b);
}

#[test]
fn abi_version_exists() {
    assert_eq!(ABI_VERSION, 1);
}

