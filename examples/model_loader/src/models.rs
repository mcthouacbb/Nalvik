use cgmath::{Vector2, Vector3};

pub struct VertexData {
    pos: Vector3<f32>,
    uv: Vector2<f32>,
    normal: Vector3<f32>,
}

impl VertexData {
    pub fn new(pos: Vector3<f32>, uv: Vector2<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, uv, normal }
    }

    pub fn pos(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn uv(&self) -> Vector2<f32> {
        self.uv
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }
}

pub mod cube;
