use cgmath::{Vector3, vec3};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    color0: Vector3<f32>,
    color1: Vector3<f32>,
}

impl Material {
    pub const MATERIAL0: Self = Self::new(vec3(0.55, 0.55, 0.55), vec3(0.9, 0.9, 0.9));
    pub const MATERIAL1: Self = Self::new(vec3(0.8, 0.7, 0.2), vec3(0.2, 0.4, 0.9));

    const fn new(color0: Vector3<f32>, color1: Vector3<f32>) -> Self {
        Self { color0, color1 }
    }

    pub fn color0(&self) -> Vector3<f32> {
        self.color0
    }

    pub fn color1(&self) -> Vector3<f32> {
        self.color1
    }
}
