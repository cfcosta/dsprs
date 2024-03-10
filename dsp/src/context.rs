use std::sync::Arc;

use crate::{llm::LanguageModel, OpenAI};

#[derive(Debug, Clone)]
pub struct Context {
    pub depth: u8,
    pub llm: Arc<dyn LanguageModel>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            depth: 0,
            llm: Arc::new(OpenAI::new("gpt-3.5-turbo")),
        }
    }
}
