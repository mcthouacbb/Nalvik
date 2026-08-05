use std::collections::HashMap;

use cgmath::{Vector2, vec2, vec3};

use crate::terrain::chunk::{TerrainChunk, generate_single_chunk};

const CHUNK_SIZE: Vector2<i32> = vec2(16, 16);
const CHUNK_LOAD_RADIUS: i32 = 4;

pub struct ChunkManager {
    chunk_map: HashMap<Vector2<i32>, TerrainChunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunk_map: HashMap::new(),
        }
    }

    // TODO: clear old chunks
    pub fn update_chunks(&mut self, camera_pos_xz: Vector2<f32>) {
        let chunk_pos_x = (camera_pos_xz.x / 16.0).round() as i32;
        let chunk_pos_z = (camera_pos_xz.y / 16.0).round() as i32;

        for dx in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
            for dz in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
                if !self
                    .chunk_map
                    .contains_key(&vec2(chunk_pos_x + dx, chunk_pos_z + dz))
                {
                    self.chunk_map.insert(
                        vec2(chunk_pos_x + dx, chunk_pos_z + dz),
                        generate_single_chunk(
                            vec2(16 * (chunk_pos_x + dx) - 8, 16 * (chunk_pos_z + dz) - 8),
                            CHUNK_SIZE,
                            vec3(1.0, 1.0, 1.0),
                        ),
                    );
                }
            }
        }
    }

    pub fn get_active_chunks(&self, camera_pos_xz: Vector2<f32>) -> Vec<&TerrainChunk> {
        let chunk_pos_x = (camera_pos_xz.x / 16.0).round() as i32;
        let chunk_pos_z = (camera_pos_xz.y / 16.0).round() as i32;

        let mut result = Vec::new();

        for dx in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
            for dz in -CHUNK_LOAD_RADIUS..=CHUNK_LOAD_RADIUS {
                result.push(
                    self.chunk_map
                        .get(&vec2(chunk_pos_x + dx, chunk_pos_z + dz))
                        .unwrap(),
                );
            }
        }

        result
    }
}
