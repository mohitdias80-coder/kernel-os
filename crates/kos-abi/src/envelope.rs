use crate::{AbiRequest, AbiResponse};

/// Wraps a request with an optional response.
#[derive(Debug, Clone)]
pub struct AbiEnvelope {
    pub request: AbiRequest,
    pub response: Option<AbiResponse>,
}

impl AbiEnvelope {
    pub fn new(request: AbiRequest) -> Self {
        Self {
            request,
            response: None,
        }
    }

    pub fn respond(&mut self, response: AbiResponse) {
        self.response = Some(response);
    }
}
