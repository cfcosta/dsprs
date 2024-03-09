use std::{any::Any, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{modules::Module, Context, Error, Signature};

#[derive(Default)]
pub struct Chain<A, B> {
    pub modules: Vec<Arc<RwLock<dyn Any>>>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

impl<A, B> Chain<A, B> {
    pub fn new() -> Self {
        Self {
            modules: vec![],
            _a: PhantomData,
            _b: PhantomData,
        }
    }
}

#[async_trait]
impl<A, B> Module for Chain<A, B>
where
    A: Signature + Send + Sync,
    B: Signature + Send + Sync,
{
    type From = A;
    type To = B;

    async fn forward(_context: Context, _input: Self::From) -> Result<Self::To, Error> {
        todo!("For each module, call the forwarding, processing the inputs during it.");
    }
}
