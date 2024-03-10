use dsp::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Signature, Serialize, Deserialize, JsonSchema, Default)]
/// Given an input question, answer it to the best of your habilities.
pub struct AnswerQuestion {
    #[port(input)]
    /// The question to be answered
    pub question: String,

    #[port(output)]
    /// The answer to the question
    pub answer: String,
}

#[derive(Signature, Default)]
/// Given a question and a detailed answer, summarize the answer.
pub struct SummarizeAnswer {
    #[port(input)]
    /// The question to be answered
    pub question: String,

    #[port(input)]
    /// The answer to the question
    pub answer: String,

    #[port(output)]
    /// A summarized version of the answer to the question
    pub summary: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    chain!(AnswerQuestion -> SummarizeAnswer);
    let _context = Context::new();
    let result = request!(qa, { question => "baby don't hurt me" }).await?;

    println!("{:?}", result);

    Ok(())
}
