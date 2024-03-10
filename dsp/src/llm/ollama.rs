use std::fmt::Debug;

use async_trait::async_trait;
use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama as OllamaRs,
};

use crate::Error;

use super::{Completion, LanguageModel};

#[derive(Debug)]
pub struct Ollama {
    model: String,
    inner: OllamaRs,
}

impl Ollama {
    pub fn new(model: &str) -> Self {
        Self {
            inner: OllamaRs::default(),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LanguageModel for Ollama {
    async fn complete(&self, prompt: &str) -> Result<Completion, Error> {
        let options = GenerationOptions::default()
            .temperature(1.0)
            .repeat_penalty(1.5)
            .top_k(25)
            .top_p(0.25);

        let res = self
            .inner
            .generate(
                GenerationRequest::new(self.model.clone(), prompt.to_string()).options(options),
            )
            .await
            .map_err(|_| Error::LLMError)?;

        dbg!(res);

        Ok(Completion)
    }
}
