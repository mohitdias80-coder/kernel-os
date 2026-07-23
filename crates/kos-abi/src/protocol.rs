/// Supported Cognitive ABI operations.
#[derive(Debug, Clone)]
pub enum AbiOperation {
    InvokeCapability,
    SpawnMission,
    SpawnAgent,
    LoadKnowledge,
    UpdateContext,
    AllocateCompute,
}
