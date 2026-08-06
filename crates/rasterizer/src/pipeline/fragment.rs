use std::marker::PhantomData;

use cgmath::Vector4;

use crate::{pipeline::vertex_to_fragment::VertexToFragment, uniform::Uniform};

pub struct FragmentShader<
    I: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    F: Fn(&I, (&U0, &U1, &U2, &U3)) -> Vector4<f32>,
> {
    shader: F,
    _marker: PhantomData<(I, U0, U1, U2, U3)>,
}

impl<
    I: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    F: Fn(&I, (&U0, &U1, &U2, &U3)) -> Vector4<f32>,
> FragmentShader<I, U0, U1, U2, U3, F>
{
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker: PhantomData,
        }
    }

    pub fn run(&self, fi: &I, uniforms: (&U0, &U1, &U2, &U3)) -> Vector4<f32> {
        (self.shader)(fi, uniforms)
    }
}
