use cgmath::{Vector2, Vector3, Vector4};

// TODO: derive macro for this to make things easier
pub trait VertexToFragment {
    fn scale_w(&mut self, scale: f32);
    fn interpolate(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self;
}

impl VertexToFragment for f32 {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        *a * barycentric.x + *b * barycentric.y + *c * barycentric.z
    }
}

impl VertexToFragment for Vector2<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}

impl VertexToFragment for Vector3<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}

impl VertexToFragment for Vector4<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}
