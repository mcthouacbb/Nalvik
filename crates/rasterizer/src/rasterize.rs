use cgmath::{Vector2, Vector3, Vector4, vec2, vec3};

use crate::{
    pipeline::{
        render_pass::{RenderPass, TriangleData},
        vertex_to_fragment::VertexToFragment,
    },
    uniform::Uniform,
};

const ONE: i32 = 256;
const HALF: i32 = ONE / 2;

const _: () = assert!(ONE & (ONE - 1) == 0 && ONE > 1);

// v should be in [-1, 1]^2
fn to_viewport(v: Vector2<f32>, viewport_size: Vector2<i32>) -> Vector2<i32> {
    vec2(
        ((v.x * 0.5 + 0.5) * (viewport_size.x * ONE) as f32 + 0.5) as i32,
        ((0.5 - v.y * 0.5) * (viewport_size.y * ONE) as f32 + 0.5) as i32,
    )
}

#[derive(Clone, Copy)]
struct EdgeFn {
    pub dx: i64,
    pub dy: i64,
    pub c: i64,
    pub not_top_left: i64,
}

impl EdgeFn {
    fn from_edge(v0: Vector2<i32>, v1: Vector2<i32>) -> Self {
        let not_top_left = (v1.y - v0.y < 0 || (v1.y - v0.y == 0 && v1.x - v0.x < 0)) as i64;

        let dx = (v1.y - v0.y) as i64;
        let dy = (v0.x - v1.x) as i64;
        let c = -v0.x as i64 * dx as i64 - v0.y as i64 * dy as i64 - not_top_left;
        Self {
            dx,
            dy,
            c,
            not_top_left,
        }
    }

    fn evaluate(&self, pos: Vector2<i32>) -> i64 {
        self.dx * pos.x as i64 + self.dy * pos.y as i64 + self.c
    }
}

pub const TILE_SIZE: i32 = 16;

#[derive(Clone, Copy)]
pub struct RasterizationInfo {
    edge_fns: [EdgeFn; 3],
    norms: [f32; 3],
}

pub fn add_triangle_to_pass<
    Vi: Sync,
    Vo: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
>(
    ndc0: &Vector4<f32>,
    ndc1: &Vector4<f32>,
    ndc2: &Vector4<f32>,
    inv_w0: f32,
    inv_w1: f32,
    inv_w2: f32,
    vo0: Vo,
    vo1: Vo,
    vo2: Vo,
    uniform_indices: [u32; 4],
    render_pass: &mut RenderPass<Vi, Vo, U0, U1, U2, U3>,
) {
    let v0 = to_viewport(ndc0.xy(), render_pass.viewport_size());
    let v1 = to_viewport(ndc1.xy(), render_pass.viewport_size());
    let v2 = to_viewport(ndc2.xy(), render_pass.viewport_size());

    let edge0 = EdgeFn::from_edge(v1, v2);
    let edge1 = EdgeFn::from_edge(v2, v0);
    let edge2 = EdgeFn::from_edge(v0, v1);

    let norm0 = 1.0 / (edge0.evaluate(v0) + edge0.not_top_left) as f32;
    let norm1 = 1.0 / (edge1.evaluate(v1) + edge1.not_top_left) as f32;
    let norm2 = 1.0 / (edge2.evaluate(v2) + edge2.not_top_left) as f32;

    let triangle_id = render_pass.add_triangle(
        RasterizationInfo {
            edge_fns: [edge0, edge1, edge2],
            norms: [norm0, norm1, norm2],
        },
        [vo0, vo1, vo2],
        [inv_w0, inv_w1, inv_w2],
        [ndc0.z, ndc1.z, ndc2.z],
        uniform_indices,
    );

    let min_x = v0.x.min(v1.x).min(v2.x);
    let max_x =
        v0.x.max(v1.x)
            .max(v2.x)
            .min(render_pass.viewport_size().x * ONE);
    let min_y = v0.y.min(v1.y).min(v2.y);
    let max_y =
        v0.y.max(v1.y)
            .max(v2.y)
            .min(render_pass.viewport_size().y * ONE);

    let tile_min_x = (min_x.max(0) + HALF) / (TILE_SIZE * ONE);
    let tile_min_y = (min_y.max(0) + HALF) / (TILE_SIZE * ONE);

    let tile_max_x = (max_x + TILE_SIZE * ONE - HALF) / (TILE_SIZE * ONE);
    let tile_max_y = (max_y + TILE_SIZE * ONE - HALF) / (TILE_SIZE * ONE);

    let mut tile_y = tile_min_y;
    while tile_y < tile_max_y && tile_y < render_pass.num_tiles().y {
        let mut tile_x = tile_min_x;
        while tile_x < tile_max_x && tile_x < render_pass.num_tiles().x {
            render_pass.add_tri_to_tile(vec2(tile_x, tile_y), triangle_id);

            tile_x += 1;
        }

        tile_y += 1;
    }
}

pub fn rasterize_tile<
    Vi: Sync,
    Vo: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
>(
    tile: Vector2<i32>,
    render_pass: &RenderPass<Vi, Vo, U0, U1, U2, U3>,
    mut pixel_fn: impl FnMut(u32, u32, Vector3<f32>, &TriangleData<Vo>),
) {
    for tri in render_pass.tile_tri_indices(tile) {
        let triangle_data = render_pass.triangle_data(*tri as usize);
        let raster_info = triangle_data.0;

        let base_x = tile.x * TILE_SIZE * ONE + HALF;
        let base_y = tile.y * TILE_SIZE * ONE + HALF;
        let max_x = ((tile.x + 1) * TILE_SIZE).min(render_pass.viewport_size().x) * ONE;
        let max_y = ((tile.y + 1) * TILE_SIZE).min(render_pass.viewport_size().y) * ONE;

        let mut e0_base = raster_info.edge_fns[0].evaluate(vec2(base_x, base_y));
        let mut e1_base = raster_info.edge_fns[1].evaluate(vec2(base_x, base_y));
        let mut e2_base = raster_info.edge_fns[2].evaluate(vec2(base_x, base_y));

        let mut y = base_y;
        while y < max_y {
            debug_assert!(y % ONE == HALF);

            let mut e0 = e0_base;
            let mut e1 = e1_base;
            let mut e2 = e2_base;
            let mut x = base_x;
            while x < max_x {
                debug_assert!(x % ONE == HALF);

                if (e0 | e1 | e2) >= 0 {
                    let barycentric = vec3(
                        (e0 + raster_info.edge_fns[0].not_top_left) as f32 * raster_info.norms[0],
                        (e1 + raster_info.edge_fns[1].not_top_left) as f32 * raster_info.norms[1],
                        (e2 + raster_info.edge_fns[2].not_top_left) as f32 * raster_info.norms[2],
                    );
                    pixel_fn(
                        (x / ONE) as u32,
                        (y / ONE) as u32,
                        barycentric,
                        &triangle_data.1,
                    );
                }

                e0 += raster_info.edge_fns[0].dx * ONE as i64;
                e1 += raster_info.edge_fns[1].dx * ONE as i64;
                e2 += raster_info.edge_fns[2].dx * ONE as i64;
                x += ONE;
            }

            e0_base += raster_info.edge_fns[0].dy * ONE as i64;
            e1_base += raster_info.edge_fns[1].dy * ONE as i64;
            e2_base += raster_info.edge_fns[2].dy * ONE as i64;
            y += ONE;
        }
    }
}
