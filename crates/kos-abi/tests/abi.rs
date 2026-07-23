use kos_abi::{AbiDispatcher, Command, ABI_VERSION};

#[test]
fn ping_returns_pong() {
let mut dispatcher = AbiDispatcher::new();

    let response = dispatcher.dispatch(Command::new("ping"));

    assert!(response.success);
    assert_eq!(response.message, "pong");
}

#[test]
fn version_returns_abi_version() {
let mut dispatcher = AbiDispatcher::new();

    let response = dispatcher.dispatch(Command::new("version"));

    assert!(response.success);
    assert_eq!(response.message, ABI_VERSION);
}

#[test]
fn unknown_command_returns_error() {
let mut dispatcher = AbiDispatcher::new();

    let response = dispatcher.dispatch(Command::new("unknown"));

    assert!(!response.success);
    assert_eq!(response.message, "Unknown ABI command");
}
