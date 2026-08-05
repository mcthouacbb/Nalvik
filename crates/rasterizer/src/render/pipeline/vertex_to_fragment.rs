use cgmath::{Vector2, Vector3, Vector4};

pub trait VertexToFragment: Copy + Sync + Send {
    fn scale_w(&mut self, scale: f32);
    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self;
    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self;
}

impl VertexToFragment for f32 {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
        a * (1.0 - t) + b * t
    }

    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        *a * barycentric.x + *b * barycentric.y + *c * barycentric.z
    }
}

impl VertexToFragment for Vector2<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
        a * (1.0 - t) + b * t
    }

    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}

impl VertexToFragment for Vector3<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
        a * (1.0 - t) + b * t
    }

    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}

impl VertexToFragment for Vector4<f32> {
    fn scale_w(&mut self, scale: f32) {
        *self *= scale;
    }

    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
        a * (1.0 - t) + b * t
    }

    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
        a * barycentric.x + b * barycentric.y + c * barycentric.z
    }
}
