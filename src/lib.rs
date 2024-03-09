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

#[derive(Debug)]
pub enum LLM {
    OpenAI,
}

pub struct LLMOptions;

impl LLM {
    pub fn gpt_4_turbo(_options: LLMOptions) -> LLM {
        todo!()
    }
}

#[macro_export]
macro_rules! chain {
    ( $( $module:ident $( ( $( $arg:expr ),* ) )? -> )* ) => {
        {
            $(
                let _ = $module::new( $( $( $arg ),* )? );
            )*
        }
    };
}

#[macro_export]
macro_rules! request {
    ($context:expr, $module:ident) => {{
        todo!()
    }};
}
