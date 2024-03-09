use dsp::*;

#[derive(Signature)]
#[signature("Given an input question, answer it to the best of your habilities.")]
pub struct AnswerQuestion {
    #[input("A question")]
    pub question: String,

    #[output("The answer to the question")]
    pub answer: String,
}

#[derive(Signature)]
#[signature("Given an input question, answer it to the best of your habilities.")]
pub struct SummarizeAnswer {
    #[input("An question")]
    pub question: String,

    #[input("An answer to the question")]
    pub answer: String,

    #[output("A summarized version of the answer to the question")]
    pub summary: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let qa = chain!(AnswerQuestion -> SummarizeAnswer);
    let context = Context::new();
    let question = "What is the meaning of life?".to_string();

    let result = request!(&context, qa).await?;

    println!("Answer: {}", result.answer);

    Ok(())
}
