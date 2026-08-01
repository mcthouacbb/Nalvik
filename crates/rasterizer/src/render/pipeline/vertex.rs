use std::marker::PhantomData;

use crate::render::{
    pipeline::{VertexOutput, vertex_to_fragment::VertexToFragment},
    uniforms::Uniforms,
};

// no dedicated vertex input struct for now
pub struct VertexShader<I, U: Uniforms, O: VertexToFragment, F: Fn(I, U) -> VertexOutput<O>> {
    shader: F,
    _marker0: PhantomData<(I, U)>,
}

impl<I, U: Uniforms, O: VertexToFragment, F: Fn(I, U) -> VertexOutput<O>> VertexShader<I, U, O, F> {
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker0: PhantomData,
        }
    }

    pub fn run(&self, vi: I, uniforms: U) -> VertexOutput<O> {
        (self.shader)(vi, uniforms)
    }
}
