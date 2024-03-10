use crate::{Ref, Signature};

#[derive(Default)]
pub struct ChainBuilder {
    inputs: Vec<Ref>,
    outputs: Vec<Ref>,
}

impl ChainBuilder {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn add_link<A: Signature, B: Signature>(&mut self, _from: Ref, _to: Ref) -> &mut Self {
        let a_inputs = A::inputs();
        let a_outputs = A::outputs();
        let b_inputs = B::inputs();
        let b_outputs = B::outputs();

        if self.inputs.is_empty() {
            self.inputs = a_inputs
        }

        if self.outputs.is_empty() {
            self.outputs = b_outputs
        }

        self
    }
}
