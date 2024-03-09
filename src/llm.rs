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
