use cgmath::{InnerSpace, Vector2, Vector3, vec2, vec3};

use crate::{render::BasicVertexData, terrain::noise::Noise};

fn triangle_color(pos0: Vector3<f32>, pos1: Vector3<f32>, pos2: Vector3<f32>) -> Vector3<f32> {
    let avg_height = (pos0.y + pos1.y + pos2.y) / 3.0;
    if avg_height < 0.3 {
        vec3(0.365, 0.702, 0.91)
    } else if avg_height < 1.5 {
        vec3(0.831, 0.761, 0.325)
    } else if avg_height < 3.5 {
        vec3(0.086, 0.651, 0.357)
    } else if avg_height < 9.5 {
        vec3(0.459, 0.329, 0.082)
    } else {
        vec3(0.7, 0.8, 0.9)
    }
}

pub struct TerrainChunk {
    mesh: Vec<[BasicVertexData; 3]>,
    noise_values: Vec<f32>,
    base_pos: Vector2<i32>,
    size: Vector2<i32>,
}

impl TerrainChunk {
    pub fn noise_idx(&self, x: i32, y: i32) -> usize {
        (y * self.size.x + x) as usize
    }

    pub fn mesh(&self) -> &Vec<[BasicVertexData; 3]> {
        &self.mesh
    }

    pub fn base_pos(&self) -> Vector2<i32> {
        self.base_pos
    }

    pub fn regen_mesh(&mut self) {
        self.mesh.clear();
        for y in 0..self.size.y - 1 {
            for x in 0..self.size.x - 1 {
                let a = vec3(x as f32, self.noise_values[self.noise_idx(x, y)], y as f32);
                let b = vec3(
                    x as f32 + 1.0,
                    self.noise_values[self.noise_idx(x + 1, y)],
                    y as f32,
                );
                let c = vec3(
                    x as f32,
                    self.noise_values[self.noise_idx(x, y + 1)],
                    y as f32 + 1.0,
                );
                let d = vec3(
                    x as f32 + 1.0,
                    self.noise_values[self.noise_idx(x + 1, y + 1)],
                    y as f32 + 1.0,
                );

                let normal1 = (c - a).cross(b - a).normalize();
                let color1 = triangle_color(a, b, c);
                self.mesh.push([
                    BasicVertexData::new(a, color1, normal1),
                    BasicVertexData::new(c, color1, normal1),
                    BasicVertexData::new(b, color1, normal1),
                ]);
                let normal2 = (b - d).cross(c - d).normalize();
                let color2 = triangle_color(b, c, d);
                self.mesh.push([
                    BasicVertexData::new(b, color2, normal2),
                    BasicVertexData::new(c, color2, normal2),
                    BasicVertexData::new(d, color2, normal2),
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
        noise_values: vec![0.0; (size.x * size.y) as usize],
        base_pos,
        size,
    };

    for y in 0..size.y {
        for x in 0..size.x {
            let idx = terrain.noise_idx(x, y);
            terrain.noise_values[idx] =
                noise.get(vec2((base_pos.x + x) as f32, (base_pos.y + y) as f32)) as f32;
        }
    }

    terrain.regen_mesh();
    terrain
}
