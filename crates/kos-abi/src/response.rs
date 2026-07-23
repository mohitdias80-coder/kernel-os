#[derive(Debug, Clone)]
pub struct AbiResponse {
    pub success: bool,
    pub message: String,
}

impl AbiResponse {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self {
            success: true,
            message: msg.into(),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            message: msg.into(),
        }
    }
}



