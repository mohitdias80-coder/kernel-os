use kos_abi::{AbiDispatcher, Command};

#[test]
fn dispatcher_accepts_multiple_commands() {
    let mut dispatcher = AbiDispatcher::new();

    let response1 = dispatcher.dispatch(Command::new("create_context"));
    assert!(response1.success);

    let response2 = dispatcher.dispatch(Command::new("spawn_agent"));
    assert!(response2.success);

    let response3 = dispatcher.dispatch(Command::new("execute"));
    assert!(response3.success);
}
