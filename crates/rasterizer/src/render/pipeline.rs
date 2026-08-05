use arrayvec::ArrayVec;
use cgmath::{Vector2, Vector3, Vector4, vec2};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::render::{
    clip,
    image::{
        format::{DepthFormat, ImageFormat, RgbaF32},
        view::ImageViewMut,
    },
    pipeline::{
        depth_state::{DepthState, DepthTest},
        fragment::FragmentShader,
        render_pass::{RenderPass, TriangleData},
        tile::TileMut,
        vertex::VertexShader,
        vertex_to_fragment::VertexToFragment,
    },
    rasterize::{TILE_SIZE, add_triangle_to_pass, rasterize_tile},
    uniform::{Uniform, Uniforms},
};

pub mod depth_state;
mod fragment;
pub mod render_pass;
mod tile;
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
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    Vs: Fn(&Vi, (&U0, &U1, &U2, &U3)) -> VertexOutput<Vo>,
    Fs: Fn(&Vo, (&U0, &U1, &U2, &U3)) -> Vector4<f32>,
> {
    vertex: VertexShader<Vi, U0, U1, U2, U3, Vo, Vs>,
    fragment: FragmentShader<Vo, U0, U1, U2, U3, Fs>,
}

impl<
    Vi: Sync,
    Vo: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
    Vs: Fn(&Vi, (&U0, &U1, &U2, &U3)) -> VertexOutput<Vo> + Sync,
    Fs: Fn(&Vo, (&U0, &U1, &U2, &U3)) -> Vector4<f32> + Sync,
> Pipeline<Vi, Vo, U0, U1, U2, U3, Vs, Fs>
{
    pub fn new(vertex_shader: Vs, fragment_shader: Fs) -> Self {
        Self {
            vertex: VertexShader::new(vertex_shader),
            fragment: FragmentShader::new(fragment_shader),
        }
    }

    pub fn begin_render_pass<'a>(
        &self,
        viewport_size: Vector2<i32>,
        uniforms: Uniforms<'a, U0, U1, U2, U3>,
    ) -> RenderPass<'a, Vo, U0, U1, U2, U3> {
        RenderPass::new(viewport_size, uniforms)
    }

    pub fn add_triangle(
        &self,
        render_pass: &mut RenderPass<Vo, U0, U1, U2, U3>,
        vi0: &Vi,
        vi1: &Vi,
        vi2: &Vi,
        uniform_indices: [u32; 4],
    ) {
        let vo0 = self
            .vertex
            .run(vi0, render_pass.uniforms().get(uniform_indices));
        let vo1 = self
            .vertex
            .run(vi1, render_pass.uniforms().get(uniform_indices));
        let vo2 = self
            .vertex
            .run(vi2, render_pass.uniforms().get(uniform_indices));

        let mut out_buf = ArrayVec::<VertexOutput<Vo>, { clip::BUF_SIZE }>::new();
        clip::clip_triangle(&vo0, &vo1, &vo2, &mut out_buf, render_pass.viewport_size());

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

            add_triangle_to_pass(
                &v0,
                &v1,
                &v2,
                inv_w0,
                inv_w1,
                inv_w2,
                vertices[0].data,
                vertices[1].data,
                vertices[2].data,
                uniform_indices,
                render_pass,
            );
        }
    }

    pub fn run<'a, T: ImageFormat + From<RgbaF32>, D: DepthFormat>(
        &self,
        render_pass: &mut RenderPass<Vo, U0, U1, U2, U3>,
        color_buffer: &mut ImageViewMut<'a, T>,
        depth_state: &mut DepthState<'a, D>,
    ) {
        assert!(
            render_pass.viewport_size()
                == vec2(color_buffer.width() as i32, color_buffer.height() as i32)
        );

        match depth_state {
            DepthState::CompareOnly(_, _) => unimplemented!(),
            DepthState::WriteOnly(_) => unimplemented!(),
            DepthState::CompareAndWrite(depth_buffer, depth_test) => self
                .run_with_depth_compare_and_write(
                    render_pass,
                    color_buffer,
                    depth_buffer,
                    *depth_test,
                ),
            DepthState::None => unimplemented!(),
        }
    }

    fn run_with_depth_compare_and_write<T: ImageFormat + From<RgbaF32>, D: DepthFormat>(
        &self,
        render_pass: &mut RenderPass<Vo, U0, U1, U2, U3>,
        color_buffer: &mut ImageViewMut<T>,
        depth_buffer: &mut ImageViewMut<D>,
        depth_test: DepthTest,
    ) {
        let mut tiles =
            Vec::with_capacity((render_pass.num_tiles().x * render_pass.num_tiles().y) as usize);

        for tile_y in 0..render_pass.num_tiles().y {
            for tile_x in 0..render_pass.num_tiles().x {
                tiles.push((
                    tile_x,
                    tile_y,
                    TileMut::new(
                        color_buffer,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                        tile_x as u32,
                        tile_y as u32,
                    ),
                    TileMut::new(
                        depth_buffer,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                        tile_x as u32,
                        tile_y as u32,
                    ),
                ));
            }
        }

        tiles
            .par_iter_mut()
            .for_each(|(tile_x, tile_y, color_buf_tile, depth_buf_tile)| {
                let tile_x = *tile_x;
                let tile_y = *tile_y;
                rasterize_tile(
                    vec2(tile_x, tile_y),
                    render_pass,
                    |mut x: u32,
                     mut y: u32,
                     barycentric: Vector3<f32>,
                     tri_data: &TriangleData<Vo>| {
                        debug_assert!(
                            x >= (TILE_SIZE * tile_x) as u32 && y >= (TILE_SIZE * tile_y) as u32
                        );
                        // get tile relative coordinates
                        x -= (TILE_SIZE * tile_x) as u32;
                        y -= (TILE_SIZE * tile_y) as u32;

                        // depth is a screen space linear (not perspective correct) interpolation of z/w
                        // this is equivalent to a taking the perspective correct interpolation of
                        // clip-space z and dividing that by the perspective correct interpolation of w
                        let depth = tri_data.z_over_w[0] * barycentric.x
                            + tri_data.z_over_w[1] * barycentric.y
                            + tri_data.z_over_w[2] * barycentric.z;
                        let new_depth = D::from(depth);
                        let old_depth = unsafe { *depth_buf_tile.get_ptr(x, y).as_ref() };

                        if depth_test.compare(&old_depth, &new_depth) {
                            unsafe {
                                *depth_buf_tile.get_ptr(x, y).as_mut() = new_depth;
                            }

                            let w = 1.0
                                / (tri_data.inv_w[0] * barycentric.x
                                    + tri_data.inv_w[1] * barycentric.y
                                    + tri_data.inv_w[2] * barycentric.z);

                            let mut fi = Vo::interpolate3(
                                &tri_data.vertex_data[0],
                                &tri_data.vertex_data[1],
                                &tri_data.vertex_data[2],
                                barycentric,
                            );
                            fi.scale_w(w);

                            let color = self
                                .fragment
                                .run(&fi, render_pass.uniforms().get(tri_data.uniform_indices));
                            unsafe {
                                *color_buf_tile.get_ptr(x, y).as_mut() = RgbaF32::new(color).into()
                            }
                        }
                    },
                );
            });
    }
}
