use std::collections::HashMap;

use cgmath::{Vector2, vec2};

use crate::terrain::chunk::{TerrainChunk, generate_single_chunk};

pub const CHUNK_SIZE: Vector2<i32> = vec2(16, 16);

pub struct ChunkManager {
    chunk_map: HashMap<Vector2<i32>, TerrainChunk>,
    render_distance: u32,
}

impl ChunkManager {
    pub fn new(render_distance: u32) -> Self {
        Self {
            chunk_map: HashMap::new(),
            render_distance,
        }
    }

    pub fn render_distance(&self) -> u32 {
        self.render_distance
    }

    pub fn update_chunks(&mut self, camera_pos_xz: Vector2<f32>) {
        let chunk_pos_x = (camera_pos_xz.x / 16.0).round() as i32;
        let chunk_pos_z = (camera_pos_xz.y / 16.0).round() as i32;

        let render_distance = self.render_distance as i32;

        for dx in -render_distance..=render_distance {
            for dz in -render_distance..=render_distance {
                if !self
                    .chunk_map
                    .contains_key(&vec2(chunk_pos_x + dx, chunk_pos_z + dz))
                {
                    self.chunk_map.insert(
                        vec2(chunk_pos_x + dx, chunk_pos_z + dz),
                        generate_single_chunk(
                            vec2(16 * (chunk_pos_x + dx) - 8, 16 * (chunk_pos_z + dz) - 8),
                            CHUNK_SIZE,
                        ),
                    );
                }
            }
        }

        self.chunk_map.retain(|key, _| {
            (key.x - chunk_pos_x).abs() <= render_distance
                || (key.y - chunk_pos_z).abs() <= render_distance
        });
    }

    pub fn get_active_chunks(&self, camera_pos_xz: Vector2<f32>) -> Vec<&TerrainChunk> {
        let chunk_pos_x = (camera_pos_xz.x / 16.0).round() as i32;
        let chunk_pos_z = (camera_pos_xz.y / 16.0).round() as i32;

        let mut result = Vec::new();

        let render_distance = self.render_distance as i32;

        for dx in -render_distance..=render_distance {
            for dz in -render_distance..=render_distance {
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
