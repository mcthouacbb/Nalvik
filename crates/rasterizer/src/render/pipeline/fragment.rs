use std::marker::PhantomData;

use cgmath::Vector4;

use crate::render::{pipeline::vertex_to_fragment::VertexToFragment, uniforms::Uniforms};

pub struct FragmentShader<I: VertexToFragment, U: Uniforms, F: Fn(I, U) -> Vector4<f32>> {
    shader: F,
    _marker: PhantomData<(I, U)>,
}

impl<I: VertexToFragment, U: Uniforms, F: Fn(I, U) -> Vector4<f32>> FragmentShader<I, U, F> {
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker: PhantomData,
        }
    }

    pub fn run(&self, fi: I, uniforms: U) -> Vector4<f32> {
        (self.shader)(fi, uniforms)
    }
}
