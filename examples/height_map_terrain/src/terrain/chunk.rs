use cgmath::{InnerSpace, Vector2, vec2, vec3};

use crate::{
    render::TerrainVertexData,
    terrain::noise::{Biome, Noise},
};

pub struct TerrainChunk {
    mesh: Vec<[TerrainVertexData; 3]>,
    noise_values: Vec<(f32, Biome)>,
    base_pos: Vector2<i32>,
    size: Vector2<i32>,
}

impl TerrainChunk {
    pub fn noise_idx(&self, x: i32, y: i32) -> usize {
        (y * self.size.x + x) as usize
    }

    pub fn mesh(&self) -> &Vec<[TerrainVertexData; 3]> {
        &self.mesh
    }

    pub fn base_pos(&self) -> Vector2<i32> {
        self.base_pos
    }

    pub fn regen_mesh(&mut self) {
        self.mesh.clear();
        for y in 0..self.size.y - 1 {
            for x in 0..self.size.x - 1 {
                let a = vec3(
                    x as f32,
                    self.noise_values[self.noise_idx(x, y)].0,
                    y as f32,
                );
                let b = vec3(
                    x as f32 + 1.0,
                    self.noise_values[self.noise_idx(x + 1, y)].0,
                    y as f32,
                );
                let c = vec3(
                    x as f32,
                    self.noise_values[self.noise_idx(x, y + 1)].0,
                    y as f32 + 1.0,
                );
                let d = vec3(
                    x as f32 + 1.0,
                    self.noise_values[self.noise_idx(x + 1, y + 1)].0,
                    y as f32 + 1.0,
                );

                let a_color = self.noise_values[self.noise_idx(x, y)].1.get_color(a);
                let b_color = self.noise_values[self.noise_idx(x + 1, y)].1.get_color(b);
                let c_color = self.noise_values[self.noise_idx(x, y + 1)].1.get_color(c);
                let d_color = self.noise_values[self.noise_idx(x + 1, y + 1)]
                    .1
                    .get_color(d);

                let normal1 = (c - a).cross(b - a).normalize();
                self.mesh.push([
                    TerrainVertexData::new(a, a_color, normal1),
                    TerrainVertexData::new(c, c_color, normal1),
                    TerrainVertexData::new(b, b_color, normal1),
                ]);
                let normal2 = (b - d).cross(c - d).normalize();
                self.mesh.push([
                    TerrainVertexData::new(b, b_color, normal2),
                    TerrainVertexData::new(c, c_color, normal2),
                    TerrainVertexData::new(d, d_color, normal2),
                ]);
            }
        }
    }
}

pub fn generate_single_chunk(base_pos: Vector2<i32>, mut size: Vector2<i32>) -> TerrainChunk {
    size.x += 1;
    size.y += 1;

    let noise = Noise::new();
    let mut terrain = TerrainChunk {
        mesh: Vec::new(),
        noise_values: vec![(0.0, Biome::Plains); (size.x * size.y) as usize],
        base_pos,
        size,
    };

    for y in 0..size.y {
        for x in 0..size.x {
            let idx = terrain.noise_idx(x, y);
            terrain.noise_values[idx] =
                noise.get(vec2((base_pos.x + x) as f32, (base_pos.y + y) as f32));
        }
    }

    terrain.regen_mesh();
    terrain
}
