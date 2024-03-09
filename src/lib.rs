use std::sync::Arc;

use thiserror::Error as ThisError;

pub use dsp_macros::Signature;

pub mod modules;

pub use modules::Module;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("TODO: Add message here")]
    LLMError,
}

pub trait Signature {
    type Inputs;
    type Outputs;
}

#[derive(Debug, Clone)]
pub struct Context {
    pub depth: u8,
    pub llm: Arc<LLM>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            depth: 0,
            llm: Arc::new(LLM::OpenAI),
        }
    }
}

pub enum LLM {
    OpenAI,
}

pub struct LLMOptions;

impl LLM {
    pub fn gpt_4_turbo(options: LLMOptions) -> LLM {
        todo!()
    }
}
