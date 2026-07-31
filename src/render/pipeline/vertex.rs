use std::marker::PhantomData;

use crate::render::pipeline::{VertexOutput, vertex_to_fragment::VertexToFragment};

// no dedicated vertex input struct for now
pub struct VertexShader<I, O: VertexToFragment, F: Fn(I) -> VertexOutput<O>> {
    shader: F,
    _marker0: PhantomData<I>,
}

impl<I, O: VertexToFragment, F: Fn(I) -> VertexOutput<O>> VertexShader<I, O, F> {
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker0: PhantomData,
        }
    }

    pub fn run(&self, vi: I) -> VertexOutput<O> {
        (self.shader)(vi)
    }
}
