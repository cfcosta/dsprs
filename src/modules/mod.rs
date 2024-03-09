use crate::Signature;
use async_trait::async_trait;

#[async_trait]
pub trait Module {
    type Signature: Signature;
}

pub struct Prediction<S: Signature> {
    pub signature: S,
}

pub struct Predict;

pub fn predict<S: Signature>(signature: S) -> Prediction<S> {
    todo!()
}

pub struct ChainOfThought;

pub fn chain_of_thought<S: Signature>(signature: S) -> Prediction<S> {
    todo!()
}
