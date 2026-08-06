use std::marker::PhantomData;

use crate::{
    pipeline::{VertexOutput, vertex_to_fragment::VertexToFragment},
    uniform::Uniform,
};

// no dedicated vertex input struct for now
pub struct VertexShader<
    I,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    O: VertexToFragment,
    F: Fn(&I, (&U0, &U1, &U2, &U3)) -> VertexOutput<O>,
> {
    shader: F,
    _marker0: PhantomData<(I, U0, U1, U2, U3)>,
}

impl<
    I,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    O: VertexToFragment,
    F: Fn(&I, (&U0, &U1, &U2, &U3)) -> VertexOutput<O>,
> VertexShader<I, U0, U1, U2, U3, O, F>
{
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker0: PhantomData,
        }
    }

    pub fn run(&self, vi: &I, uniforms: (&U0, &U1, &U2, &U3)) -> VertexOutput<O> {
        (self.shader)(vi, uniforms)
    }
}
