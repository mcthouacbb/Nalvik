use cgmath::{InnerSpace, Vector2, Vector3, vec3};
use noise::NoiseFn;

use crate::render::BasicVertexData;

pub struct Terrain {
    mesh: Vec<[BasicVertexData; 3]>,
    noise_values: Vec<f32>,
    base_pos: Vector2<i32>,
    size: Vector2<i32>,
    color: Vector3<f32>,
}

impl Terrain {
    pub fn noise_idx(&self, x: i32, y: i32) -> usize {
        (y * self.size.x + x) as usize
    }

    pub fn mesh(&self) -> &Vec<[BasicVertexData; 3]> {
        &self.mesh
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
                self.mesh.push([
                    BasicVertexData::new(a, self.color, normal1),
                    BasicVertexData::new(c, self.color, normal1),
                    BasicVertexData::new(b, self.color, normal1),
                ]);
                let normal2 = (b - d).cross(c - d).normalize();
                self.mesh.push([
                    BasicVertexData::new(b, self.color, normal2),
                    BasicVertexData::new(c, self.color, normal2),
                    BasicVertexData::new(d, self.color, normal2),
                ]);
            }
        }
    }
}

pub fn generate_terrain(
    base_pos: Vector2<i32>,
    size: Vector2<i32>,
    color: Vector3<f32>,
) -> Terrain {
    let noise = noise::OpenSimplex::new(0x2A8d2F39);
    let mut terrain = Terrain {
        mesh: Vec::new(),
        noise_values: vec![0.0; (size.x * size.y) as usize],
        base_pos,
        size,
        color,
    };

    for y in 0..size.y {
        for x in 0..size.x {
            let idx = terrain.noise_idx(x, y);
            terrain.noise_values[idx] =
                noise.get([(x + base_pos.x) as f64 * 0.2, (y + base_pos.y) as f64 * 0.2]) as f32
                    * 5.0;
        }
    }

    terrain.regen_mesh();
    terrain
}
