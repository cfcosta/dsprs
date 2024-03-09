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
    ( $( $module:ident )+ ) => {{
        ()
    }};
    ( $( $module:ident )+ $( -> $module_tail:ident )+ ) => {{
        chain!($( $module )+);
    }};
}

#[macro_export]
macro_rules! request {
    ($context:expr, $module:ident) => {
        async {
            use ollama_rs::{
                generation::{completion::request::GenerationRequest, options::GenerationOptions},
                Ollama,
            };

            let model = "gemma:2b".to_string();
            let prompt = "What is love?".to_string();

            let options = GenerationOptions::default()
                .temperature(0.2)
                .repeat_penalty(1.5)
                .top_k(25)
                .top_p(0.25);

            let ollama = Ollama::default();

            let res = ollama
                .generate(GenerationRequest::new(model, prompt).options(options))
                .await;

            if let Ok(ref res) = res {
                println!("{}", res.response);
            }

            Ok(res.map_err(|_| dsp::Error::LLMError))
        }
    };
}
