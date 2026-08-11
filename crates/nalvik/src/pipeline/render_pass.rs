use cgmath::{Vector2, vec2};

use crate::{
    pipeline::vertex_to_fragment::VertexToFragment,
    rasterize::{RasterizationInfo, TILE_SIZE},
    uniform::{Uniform, Uniforms},
    util::chunked_vec::ChunkedVec,
};

struct TileData {
    tri_indices: Vec<u32>,
}

pub struct TriangleData<Vo: VertexToFragment> {
    pub vertex_data: [Vo; 3],
    pub inv_w: [f32; 3],
    pub z_over_w: [f32; 3],
    pub uniform_indices: [u32; 4],
}

pub const TRI_BUF_CHUNK_SIZE: usize = 1024;

pub struct RenderPass<
    'a,
    Vi: Sync,
    Vo: VertexToFragment,
    U0: Uniform,
    U1: Uniform,
    U2: Uniform,
    U3: Uniform,
> {
    raw_triangles: ChunkedVec<(&'a Vi, &'a Vi, &'a Vi, [u32; 4]), TRI_BUF_CHUNK_SIZE>,

    triangles: ChunkedVec<(RasterizationInfo, TriangleData<Vo>), TRI_BUF_CHUNK_SIZE>,
    tiles: Vec<TileData>,
    viewport_size: Vector2<i32>,
    num_tiles: Vector2<i32>,

    uniforms: Uniforms<'a, U0, U1, U2, U3>,
}

impl<'a, Vi: Sync, Vo: VertexToFragment, U0: Uniform, U1: Uniform, U2: Uniform, U3: Uniform>
    RenderPass<'a, Vi, Vo, U0, U1, U2, U3>
{
    pub fn new(viewport_size: Vector2<i32>, uniforms: Uniforms<'a, U0, U1, U2, U3>) -> Self {
        let num_tiles_x = (viewport_size.x + TILE_SIZE - 1) / TILE_SIZE;
        let num_tiles_y = (viewport_size.y + TILE_SIZE - 1) / TILE_SIZE;
        let mut tiles = Vec::with_capacity((num_tiles_x * num_tiles_y) as usize);
        for _ in 0..num_tiles_x * num_tiles_y {
            tiles.push(TileData {
                tri_indices: Vec::with_capacity(256),
            });
        }
        Self {
            raw_triangles: ChunkedVec::with_capacity(64),
            triangles: ChunkedVec::with_capacity(64),
            tiles,
            viewport_size,
            num_tiles: vec2(num_tiles_x, num_tiles_y),
            uniforms,
        }
    }

    pub fn viewport_size(&self) -> Vector2<i32> {
        self.viewport_size
    }

    pub fn num_tiles(&self) -> Vector2<i32> {
        self.num_tiles
    }

    pub fn uniforms(&self) -> &Uniforms<'a, U0, U1, U2, U3> {
        &self.uniforms
    }

    pub fn raw_triangles(&self) -> &ChunkedVec<(&'a Vi, &'a Vi, &'a Vi, [u32; 4]), 1024> {
        &self.raw_triangles
    }

    pub fn triangle_data(&self, idx: usize) -> &(RasterizationInfo, TriangleData<Vo>) {
        &self.triangles[idx]
    }

    pub fn tile_tri_indices(&self, tile: Vector2<i32>) -> &[u32] {
        &self.tiles[(tile.y * self.num_tiles().x + tile.x) as usize].tri_indices
    }

    pub fn add_raw_triangle(
        &mut self,
        vi0: &'a Vi,
        vi1: &'a Vi,
        vi2: &'a Vi,
        uniform_indices: [u32; 4],
    ) {
        self.raw_triangles.push((vi0, vi1, vi2, uniform_indices));
    }

    pub fn add_triangle(
        &mut self,
        rasterization_info: RasterizationInfo,
        vertex_data: [Vo; 3],
        inv_w: [f32; 3],
        z_over_w: [f32; 3],
        uniform_indices: [u32; 4],
    ) -> u32 {
        let id = self.triangles.len() as u32;
        self.triangles.push((
            rasterization_info,
            TriangleData {
                vertex_data,
                inv_w,
                z_over_w,
                uniform_indices,
            },
        ));
        id
    }

    pub fn add_tri_to_tile(&mut self, tile: Vector2<i32>, tri_id: u32) {
        let stride = self.num_tiles().x;
        self.tiles[(tile.y * stride + tile.x) as usize]
            .tri_indices
            .push(tri_id);
    }
}
