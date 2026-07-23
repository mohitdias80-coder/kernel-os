use crate::{AbiCall, AbiResponse, Command};
use kos_orchestrator::Orchestrator;

/// Dispatches ABI commands into Kernel OS.
#[derive(Debug)]
pub struct AbiDispatcher {
    orchestrator: Orchestrator,
}

impl AbiDispatcher {
pub fn new() -> Self {
    Self {
        orchestrator: Orchestrator::new(),
    }
}

pub fn dispatch(&mut self, command: Command) -> AbiResponse {
        let _call = match command.name() {
            "ping" => return AbiResponse::ok("pong"),

            "version" => return AbiResponse::ok(crate::ABI_VERSION),

            "create_context" => AbiCall::CreateContext,
            "spawn_agent" => AbiCall::SpawnAgent,
            "store_knowledge" => AbiCall::StoreKnowledge,
            "execute" => AbiCall::ExecuteCommand,

            _ => return AbiResponse::error("Unknown ABI command"),
        };

        // Forward into the Kernel OS execution pipeline.
self.orchestrator.tick();

        AbiResponse::ok("Command accepted")
    }
}
