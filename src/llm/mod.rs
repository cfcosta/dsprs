use std::fmt::Debug;

use async_trait::async_trait;

use crate::Error;

mod ollama;
pub use ollama::Ollama;

#[derive(Debug)]
pub struct Completion;

#[async_trait]
pub trait LanguageModel
where
    Self: Debug,
{
    async fn complete(&self, prompt: &str) -> Result<Completion, Error>;
}
