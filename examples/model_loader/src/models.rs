pub mod cube;
pub mod obj;

use cgmath::{Vector2, Vector3, vec2, vec3, vec4};

use crate::material::Material;

#[derive(Clone, Copy)]
pub struct VertexData {
    pos: Vector3<f32>,
    uv: Vector2<f32>,
    normal: Vector3<f32>,
}

impl VertexData {
    pub fn new(pos: Vector3<f32>, uv: Vector2<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, uv, normal }
    }

    pub fn pos(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn uv(&self) -> Vector2<f32> {
        self.uv
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }
}

impl Default for VertexData {
    fn default() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            uv: vec2(0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
        }
    }
}

pub enum ModelPath {
    File(String),
    Builtin(String),
}

pub fn load_model(path: &ModelPath) -> Vec<(Vec<[VertexData; 3]>, Material)> {
    match path {
        ModelPath::File(path) => obj::load_obj_model(path),
        ModelPath::Builtin(name) => match name.as_str() {
            "cube" => {
                let mut material = Material::try_load_from_file("assets/checker.png");
                if material.is_err() {
                    eprintln!("Could not load assets/checker.png");
                    material = Ok(Material::solid_color(vec4(1.0, 0.0, 1.0, 1.0)));
                }
                vec![(cube::get_cube_model().to_vec(), material.unwrap())]
            }
            _ => {
                eprintln!("Unknown builtin model {}", name);
                Vec::new()
            }
        },
    }
}
