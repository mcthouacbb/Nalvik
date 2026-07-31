use std::marker::PhantomData;

use cgmath::Vector4;

use crate::render::pipeline::vertex_to_fragment::VertexToFragment;

pub struct FragmentShader<I: VertexToFragment, F: Fn(I) -> Vector4<f32>> {
    shader: F,
    _marker: PhantomData<I>,
}

impl<I: VertexToFragment, F: Fn(I) -> Vector4<f32>> FragmentShader<I, F> {
    pub fn new(shader: F) -> Self {
        Self {
            shader,
            _marker: PhantomData,
        }
    }

    pub fn run(&self, fi: I) -> Vector4<f32> {
        (self.shader)(fi)
    }
}
