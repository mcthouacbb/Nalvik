use cgmath::{Matrix4, Rad, Vector2, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, rotation: Vector2<f32>) -> Self {
        Self { position, rotation }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_x(-Rad(self.rotation.x))
            * Matrix4::from_angle_y(-Rad(self.rotation.y))
            * Matrix4::from_translation(-self.position)
    }
}
