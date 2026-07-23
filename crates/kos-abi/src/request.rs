/// Standard request flowing through the Cognitive ABI.
#[derive(Debug, Clone)]
pub struct AbiRequest {
    pub id: u64,
    pub operation: String,
    pub payload: String,
}

impl AbiRequest {
    pub fn new(
        id: u64,
        operation: impl Into<String>,
        payload: impl Into<String>,
    ) -> Self {
        Self {
            id,
            operation: operation.into(),
            payload: payload.into(),
        }
    }
}
