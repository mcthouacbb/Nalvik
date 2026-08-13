use cgmath::{Matrix4, Rad, Vector4, perspective};

#[rustfmt::skip]
pub const PERSPECTIVE_CORRECTION: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0
);

pub fn perspective_proj(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    PERSPECTIVE_CORRECTION * perspective(Rad(fovy), aspect, near, far)
}

pub fn oblique_projection(
    fovy: f32,
    aspect: f32,
    near_plane: Vector4<f32>,
    far_k: f32,
) -> Matrix4<f32> {
    let f = 1.0 / (fovy / 2.0).tan();

    let c0r0 = f / aspect;
    let c0r1 = 0.0;
    let c0r2 = far_k * near_plane.x;
    let c0r3 = 0.0;

    let c1r0 = 0.0;
    let c1r1 = f;
    let c1r2 = far_k * near_plane.y;
    let c1r3 = 0.0;

    let c2r0 = 0.0;
    let c2r1 = 0.0;
    let c2r2 = far_k * near_plane.z;
    let c2r3 = -1.0;

    let c3r0 = 0.0;
    let c3r1 = 0.0;
    let c3r2 = far_k * near_plane.w;
    let c3r3 = 0.0;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new(
        c0r0, c0r1, c0r2, c0r3,
        c1r0, c1r1, c1r2, c1r3,
        c2r0, c2r1, c2r2, c2r3,
        c3r0, c3r1, c3r2, c3r3,
    )
}
