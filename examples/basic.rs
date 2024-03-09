use dsp::*;

#[derive(Signature, Default)]
#[signature("Given an input question, answer it to the best of your habilities.")]
pub struct AnswerQuestion {
    #[input("A question")]
    pub question: String,

    #[output("The answer to the question")]
    pub answer: String,
}

#[derive(Signature, Default)]
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
    chain!(AnswerQuestion -> SummarizeAnswer);
    let _context = Context::new();
    let result = request!(qa, { question => "baby don't hurt me" }).await?;

    println!("{:?}", result);

    Ok(())
}
