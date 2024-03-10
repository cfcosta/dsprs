use async_trait::async_trait;

use crate::{Context, Error, Signature};

mod chain;
mod chain_of_thought;
mod predict;

pub use self::{
    chain::*,
    chain_of_thought::{chain_of_thought, ChainOfThought},
    predict::{predict, Predict},
};

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
