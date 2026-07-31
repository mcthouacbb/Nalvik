use cgmath::{Vector2, Vector3, Vector4};

use crate::render::{
    pipeline::{
        fragment::FragmentShader, vertex::VertexShader, vertex_to_fragment::VertexToFragment,
    },
    rasterize,
};

mod fragment;
mod vertex;
mod vertex_to_fragment;

pub struct VertexOutput<O: VertexToFragment> {
    pub position: Vector4<f32>,
    pub data: O,
}

pub struct Pipeline<
    Vi,
    Vo: VertexToFragment,
    Vs: Fn(Vi) -> VertexOutput<Vo>,
    Fs: Fn(Vo) -> Vector4<f32>,
> {
    vertex: VertexShader<Vi, Vo, Vs>,
    fragment: FragmentShader<Vo, Fs>,
}

impl<Vi, Vo: VertexToFragment, Vs: Fn(Vi) -> VertexOutput<Vo>, Fs: Fn(Vo) -> Vector4<f32>>
    Pipeline<Vi, Vo, Vs, Fs>
{
    pub fn new(vertex_shader: Vs, fragment_shader: Fs) -> Self {
        Self {
            vertex: VertexShader::new(vertex_shader),
            fragment: FragmentShader::new(fragment_shader),
        }
    }

    pub fn run(
        &self,
        vi0: Vi,
        vi1: Vi,
        vi2: Vi,
        viewport_size: Vector2<i32>,
        mut pixel_fn: impl FnMut(u32, u32, Vector4<f32>),
    ) {
        let mut vo0 = self.vertex.run(vi0);
        let mut vo1 = self.vertex.run(vi1);
        let mut vo2 = self.vertex.run(vi2);

        let pos0 = vo0.position;
        let pos1 = vo1.position;
        let pos2 = vo2.position;

        let inv_w0 = 1.0 / pos0.w;
        let inv_w1 = 1.0 / pos1.w;
        let inv_w2 = 1.0 / pos2.w;

        let v0 = pos0 * inv_w0;
        let v1 = pos1 * inv_w1;
        let v2 = pos2 * inv_w2;

        vo0.data.scale_w(inv_w0);
        vo1.data.scale_w(inv_w1);
        vo2.data.scale_w(inv_w2);

        rasterize::rasterize_triangle(
            v0.xy(),
            v1.xy(),
            v2.xy(),
            viewport_size,
            |x: u32, y: u32, barycentric: Vector3<f32>| {
                let w = 1.0
                    / (inv_w0 * barycentric.x + inv_w1 * barycentric.y + inv_w2 * barycentric.z);

                let mut fi = Vo::interpolate(&vo0.data, &vo1.data, &vo2.data, barycentric);
                fi.scale_w(w);

                let color = self.fragment.run(fi);
                pixel_fn(x, y, color);
            },
        );
    }
}
