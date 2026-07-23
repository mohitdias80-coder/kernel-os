use crate::{AbiCall, AbiResponse, Command};

/// Dispatches ABI commands into Kernel OS.
#[derive(Debug, Default)]
pub struct AbiDispatcher;

impl AbiDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn dispatch(&self, command: Command) -> AbiResponse {
        let _call = match command.name() {
            "ping" => return AbiResponse::ok("pong"),

            "version" => return AbiResponse::ok(crate::ABI_VERSION),

            "create_context" => AbiCall::CreateContext,
            "spawn_agent" => AbiCall::SpawnAgent,
            "store_knowledge" => AbiCall::StoreKnowledge,
            "execute" => AbiCall::ExecuteCommand,

            _ => return AbiResponse::error("Unknown ABI command"),
        };

        AbiResponse::ok("Command accepted")
    }
}
