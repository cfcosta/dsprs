pub use dsp_macros::Signature;

mod context;
mod error;
mod llm;
mod modules;

pub use self::{context::Context, error::Error, llm::LLM, modules::Module};

pub trait Signature {
    type Inputs;
    type Outputs;
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
                .temperature(1.0)
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
