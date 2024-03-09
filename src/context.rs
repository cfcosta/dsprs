use std::sync::Arc;

use crate::LLM;

#[derive(Debug, Clone)]
pub struct Context {
    pub depth: u8,
    pub llm: Arc<LLM>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            depth: 0,
            llm: Arc::new(LLM::OpenAI),
        }
    }
}
