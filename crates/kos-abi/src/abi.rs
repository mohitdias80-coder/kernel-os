#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbiCall {
    CreateMission,
    DestroyMission,

    CreateContext,
    DestroyContext,

    SpawnAgent,
    StopAgent,

    RegisterCapability,
    AcquireCapability,

    StoreKnowledge,
    QueryKnowledge,

    CheckPermission,

    ExecuteCommand,
}
