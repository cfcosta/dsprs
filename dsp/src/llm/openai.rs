use crate::*;
use async_openai::{
    config::{AzureConfig, Config, OpenAIConfig},
    types::CreateChatCompletionRequest,
    Client,
};
use async_trait::async_trait;
use std::fmt::Debug;

pub type OpenAI = OpenAIInner<OpenAIConfig>;
pub type AzureOpenAI = OpenAIInner<AzureConfig>;

#[derive(Debug)]
pub struct OpenAIInner<C: Config> {
    model: String,
    client: Client<C>,
}

impl<C: Config> OpenAIInner<C> {
    pub fn new(model: &str) -> OpenAI {
        let config = OpenAIConfig::new().with_api_key(env!("OPENAI_API_KEY"));
        let client = Client::with_config(config);

        OpenAI {
            client,
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl<C: Config + Debug + Send + Sync> LanguageModel for OpenAIInner<C> {
    async fn complete(&self, prompt: &str) -> Result<Completion, Error> {
        println!("{}", prompt);

        let request = CreateChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![],
            ..Default::default()
        };

        let response = self
            .client
            .chat()
            .create(request) // Make the API call in that "group"
            .await
            .unwrap();

        dbg!(response);

        return Err(Error::LLMError);
    }
}
