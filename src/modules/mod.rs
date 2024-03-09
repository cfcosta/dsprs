use crate::{Context, Error, Signature};
use async_trait::async_trait;

mod chain;
pub use chain::Chain;

#[async_trait]
pub trait Module {
    type From: Signature + Send + Sync;
    type To: Signature + Send + Sync;

    async fn forward(context: Context, input: Self::From) -> Result<Prediction<Self::To>, Error>;
}

pub struct Prediction<S: Signature> {
    pub signature: S,
}

impl<S: Signature> Prediction<S> {
    pub fn new(signature: S) -> Self {
        Self { signature }
    }
}

pub struct Predict;

pub fn predict<S: Signature>(_signature: S) -> Prediction<S> {
    todo!()
}

pub struct ChainOfThought;

pub fn chain_of_thought<S: Signature>(_signature: S) -> Prediction<S> {
    todo!()
}
