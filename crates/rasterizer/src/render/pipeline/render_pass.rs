use cgmath::{Vector2, vec2};

use crate::render::{
    pipeline::vertex_to_fragment::VertexToFragment,
    rasterize::{RasterizationInfo, TILE_SIZE},
    uniform::{Uniform, Uniforms},
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

pub struct RenderPass<'a, Vo: VertexToFragment, U0: Uniform, U1: Uniform, U2: Uniform, U3: Uniform>
{
    triangles: Vec<(RasterizationInfo, TriangleData<Vo>)>,
    tiles: Vec<TileData>,
    viewport_size: Vector2<i32>,
    num_tiles: Vector2<i32>,

    uniforms: Uniforms<'a, U0, U1, U2, U3>,
}

impl<'a, Vo: VertexToFragment, U0: Uniform, U1: Uniform, U2: Uniform, U3: Uniform>
    RenderPass<'a, Vo, U0, U1, U2, U3>
{
    pub fn new(viewport_size: Vector2<i32>, uniforms: Uniforms<'a, U0, U1, U2, U3>) -> Self {
        let num_tiles_x = (viewport_size.x + TILE_SIZE - 1) / TILE_SIZE;
        let num_tiles_y = (viewport_size.y + TILE_SIZE - 1) / TILE_SIZE;
        let mut tiles = Vec::with_capacity((num_tiles_x * num_tiles_y) as usize);
        for _ in 0..num_tiles_x * num_tiles_y {
            tiles.push(TileData {
                tri_indices: Vec::new(),
            });
        }
        Self {
            triangles: Vec::new(),
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

    pub fn triangle_data(&self, idx: usize) -> &(RasterizationInfo, TriangleData<Vo>) {
        &self.triangles[idx]
    }

    pub fn tile_tri_indices(&self, tile: Vector2<i32>) -> &[u32] {
        &self.tiles[(tile.y * self.num_tiles().x + tile.x) as usize].tri_indices
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
