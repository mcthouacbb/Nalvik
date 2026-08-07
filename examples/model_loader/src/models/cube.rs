use cgmath::{vec2, vec3};

use crate::models::VertexData;

pub fn cube_model() -> [[VertexData; 3]; 12] {
    // right handed coordinates
    [
        // +z face
        [
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec2(0.0, 0.0), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec2(1.0, 0.0), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec2(1.0, 1.0), vec3(0.0, 0.0, 1.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, 0.5), vec2(1.0, 1.0), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec2(0.0, 1.0), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec2(0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        ],
        // -z face
        [
            VertexData::new(vec3(0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec2(1.0, 0.0), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(0.0, 0.0, -1.0)),
        ],
        [
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec2(0.0, 1.0), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(0.0, 0.0, -1.0)),
        ],
        // +x face
        [
            VertexData::new(vec3(0.5, -0.5, 0.5), vec2(0.0, 0.0), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec2(1.0, 0.0), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(1.0, 0.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec2(0.0, 1.0), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec2(0.0, 0.0), vec3(1.0, 0.0, 0.0)),
        ],
        // -x face
        [
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec2(1.0, 0.0), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec2(1.0, 1.0), vec3(-1.0, 0.0, 0.0)),
        ],
        [
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec2(1.0, 1.0), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec2(0.0, 1.0), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(-1.0, 0.0, 0.0)),
        ],
        // +y face
        [
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec2(0.0, 0.0), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec2(1.0, 0.0), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(0.0, 1.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, -0.5), vec2(1.0, 1.0), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec2(0.0, 1.0), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec2(0.0, 0.0), vec3(0.0, 1.0, 0.0)),
        ],
        // -y face
        [
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec2(1.0, 0.0), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec2(1.0, 1.0), vec3(0.0, -1.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, -0.5, 0.5), vec2(1.0, 1.0), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec2(0.0, 1.0), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec2(0.0, 0.0), vec3(0.0, -1.0, 0.0)),
        ],
    ]
}
