use cgmath::Vector3;

pub trait VertexToFragment: Copy + Sync + Send {
    fn scale_w(&mut self, scale: f32);
    fn interpolate2(a: &Self, b: &Self, t: f32) -> Self;
    fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self;
}

impl VertexToFragment for () {
    fn scale_w(&mut self, _scale: f32) {}
    fn interpolate2(_a: &Self, _b: &Self, _t: f32) -> Self {}
    fn interpolate3(_a: &Self, _b: &Self, _c: &Self, _barycentric: Vector3<f32>) -> Self {}
}
