use kos_core::ContextId;

use crate::ContextMetadata;

/// A Context Space owns the execution state of a Mission.
///
/// This is the Kernel OS equivalent of a process address space,
/// but specialized for AI workloads.
#[derive(Debug, Clone)]
pub struct ContextSpace {
    id: ContextId,

    metadata: ContextMetadata,

    /// Active reasoning context.
    active_context: String,

    /// Working memory.
    working_memory: Vec<String>,

    /// Long-term semantic memory (placeholder for MVP).
    semantic_memory: Vec<String>,

    /// Prompt cache.
    prompt_cache: Vec<String>,
}

impl ContextSpace {
    pub fn new(id: ContextId, name: impl Into<String>) -> Self {
        Self {
            id,
            metadata: ContextMetadata::new(name),
            active_context: String::new(),
            working_memory: Vec::new(),
            semantic_memory: Vec::new(),
            prompt_cache: Vec::new(),
        }
    }

    pub fn id(&self) -> ContextId {
        self.id
    }

    pub fn metadata(&self) -> &ContextMetadata {
        &self.metadata
    }

    pub fn active_context(&self) -> &str {
        &self.active_context
    }

    pub fn set_active_context(&mut self, value: impl Into<String>) {
        self.active_context = value.into();
        self.metadata.touch();
    }

    pub fn working_memory(&self) -> &[String] {
        &self.working_memory
    }

    pub fn push_working_memory(&mut self, value: impl Into<String>) {
        self.working_memory.push(value.into());
        self.metadata.touch();
    }

    pub fn semantic_memory(&self) -> &[String] {
        &self.semantic_memory
    }

    pub fn push_semantic_memory(&mut self, value: impl Into<String>) {
        self.semantic_memory.push(value.into());
        self.metadata.touch();
    }

    pub fn prompt_cache(&self) -> &[String] {
        &self.prompt_cache
    }

    pub fn push_prompt_cache(&mut self, value: impl Into<String>) {
        self.prompt_cache.push(value.into());
        self.metadata.touch();
    }
}
