use arrayvec::ArrayVec;
use cgmath::{Vector2, Vector3, Vector4};

use crate::render::{
    clip,
    image::{
        format::{DepthFormat, ImageFormat, RgbaF32},
        view::ImageViewMut,
    },
    pipeline::{
        depth_state::DepthState, fragment::FragmentShader, vertex::VertexShader,
        vertex_to_fragment::VertexToFragment,
    },
    rasterize,
    uniforms::Uniforms,
};

pub mod depth_state;
mod fragment;
mod vertex;
pub mod vertex_to_fragment;

#[derive(Clone, Copy)]
pub struct VertexOutput<O: VertexToFragment> {
    pub position: Vector4<f32>,
    pub data: O,
}

impl<O: VertexToFragment> VertexOutput<O> {
    pub fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            position: a.position * (1.0 - t) + b.position * t,
            data: O::interpolate2(&a.data, &b.data, t),
        }
    }
}

pub struct Pipeline<
    Vi,
    Vo: VertexToFragment,
    Vu: Uniforms,
    Fu: Uniforms,
    Vs: Fn(&Vi, Vu) -> VertexOutput<Vo>,
    Fs: Fn(&Vo, Fu) -> Vector4<f32>,
> {
    vertex: VertexShader<Vi, Vu, Vo, Vs>,
    fragment: FragmentShader<Vo, Fu, Fs>,
}

impl<
    Vi,
    Vo: VertexToFragment,
    Vu: Uniforms,
    Fu: Uniforms,
    Vs: Fn(&Vi, Vu) -> VertexOutput<Vo>,
    Fs: Fn(&Vo, Fu) -> Vector4<f32>,
> Pipeline<Vi, Vo, Vu, Fu, Vs, Fs>
{
    pub fn new(vertex_shader: Vs, fragment_shader: Fs) -> Self {
        Self {
            vertex: VertexShader::new(vertex_shader),
            fragment: FragmentShader::new(fragment_shader),
        }
    }

    pub fn run<T: ImageFormat + From<RgbaF32>, D: DepthFormat>(
        &self,
        vertex_uniforms: Vu,
        fragment_uniforms: Fu,
        vi0: &Vi,
        vi1: &Vi,
        vi2: &Vi,
        viewport_size: Vector2<i32>,
        color_buffer: &mut ImageViewMut<T>,
        depth_state: &mut DepthState<D>,
    ) {
        let vo0 = self.vertex.run(vi0, vertex_uniforms);
        let vo1 = self.vertex.run(vi1, vertex_uniforms);
        let vo2 = self.vertex.run(vi2, vertex_uniforms);

        let mut out_buf = ArrayVec::<VertexOutput<Vo>, { clip::BUF_SIZE }>::new();
        clip::clip_triangle(&vo0, &vo1, &vo2, &mut out_buf, viewport_size);

        for vertices in out_buf.chunks_exact_mut(3) {
            let inv_w0 = 1.0 / vertices[0].position.w;
            let inv_w1 = 1.0 / vertices[1].position.w;
            let inv_w2 = 1.0 / vertices[2].position.w;

            let v0 = vertices[0].position * inv_w0;
            let v1 = vertices[1].position * inv_w1;
            let v2 = vertices[2].position * inv_w2;

            vertices[0].data.scale_w(inv_w0);
            vertices[1].data.scale_w(inv_w1);
            vertices[2].data.scale_w(inv_w2);

            rasterize::rasterize_triangle(
                v0.xy(),
                v1.xy(),
                v2.xy(),
                viewport_size,
                |x: u32, y: u32, barycentric: Vector3<f32>| {
                    // depth is a screen space linear (not perspective correct) interpolation of z/w
                    // this is equivalent to a taking the perspective correct interpolation of
                    // clip-space z and dividing that by the perspective correct interpolation of w
                    let depth = v0.z * barycentric.x + v1.z * barycentric.y + v2.z * barycentric.z;
                    if depth_state.keep_fragment(x, y, depth) {
                        let w = 1.0
                            / (inv_w0 * barycentric.x
                                + inv_w1 * barycentric.y
                                + inv_w2 * barycentric.z);

                        let mut fi = Vo::interpolate3(
                            &vertices[0].data,
                            &vertices[1].data,
                            &vertices[2].data,
                            barycentric,
                        );
                        fi.scale_w(w);

                        let color = self.fragment.run(&fi, fragment_uniforms);
                        *color_buffer.get_mut(x, y) = RgbaF32::new(color).into();
                    }
                },
            );
        }
    }
}
