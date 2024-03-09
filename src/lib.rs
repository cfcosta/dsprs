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
    ( $( $from:ident -> $to:expr ),* $(,)?) => {{
        println!("Creating chain from {} to {}", stringify!($($from)*), stringify!($($to)*));
    }};
}

#[macro_export]
macro_rules! params {
    ( $( $key:ident => $value:expr ),* $(,)? ) => {{
        let mut params = ::std::collections::HashMap::new();
        $(
            params.insert(stringify!($key), $value);
        )*
        params
    }}
}

#[macro_export]
macro_rules! request {
    ($context:expr, $module:ident, { $($key:ident => $value:expr),* $(,)? }) => {
        async {
            use ollama_rs::{
                generation::{completion::request::GenerationRequest, options::GenerationOptions},
                Ollama,
            };


            let mut params = ::dsp::params!($($key => $value)*);
            let prompt = params
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>().join(", ");

            println!("{}", prompt);

            let model = "gemma:2b".to_string();

            let options = GenerationOptions::default()
                .temperature(0.2)
                .repeat_penalty(1.5)
                .top_k(25)
                .top_p(0.25);

            let ollama = Ollama::default();

            let res = ollama
                .generate(GenerationRequest::new(model, prompt).options(options))
                .await;

            Ok(res.map_err(|_| dsp::Error::LLMError))
        }
    };
}
