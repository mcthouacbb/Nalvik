use cgmath::{Matrix4, Rad, Vector2, Vector3, prelude::*, vec2, vec3, vec4};
use nalvik::{
    Image2d,
    format::{DepthF32, RgbaU8},
};

use crate::{material::Material, render::VertexData};

fn cube_model() -> [[VertexData; 3]; 12] {
    // right handed coordinates
    [
        // +z face
        [
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec3(0.0, 0.0, 1.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, 0.5), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec3(0.0, 0.0, 1.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec3(0.0, 0.0, 1.0)),
        ],
        // -z face
        [
            VertexData::new(vec3(0.5, -0.5, -0.5), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec3(0.0, 0.0, -1.0)),
        ],
        [
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec3(0.0, 0.0, -1.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec3(0.0, 0.0, -1.0)),
        ],
        // +x face
        [
            VertexData::new(vec3(0.5, -0.5, 0.5), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec3(1.0, 0.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, -0.5), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec3(1.0, 0.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec3(1.0, 0.0, 0.0)),
        ],
        // -x face
        [
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec3(-1.0, 0.0, 0.0)),
        ],
        [
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec3(-1.0, 0.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec3(-1.0, 0.0, 0.0)),
        ],
        // +y face
        [
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, 0.5), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(0.5, 0.5, -0.5), vec3(0.0, 1.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, 0.5, -0.5), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, -0.5), vec3(0.0, 1.0, 0.0)),
            VertexData::new(vec3(-0.5, 0.5, 0.5), vec3(0.0, 1.0, 0.0)),
        ],
        // -y face
        [
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, -0.5), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(0.5, -0.5, 0.5), vec3(0.0, -1.0, 0.0)),
        ],
        [
            VertexData::new(vec3(0.5, -0.5, 0.5), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, 0.5), vec3(0.0, -1.0, 0.0)),
            VertexData::new(vec3(-0.5, -0.5, -0.5), vec3(0.0, -1.0, 0.0)),
        ],
    ]
}

fn rectangular_prism(
    pos: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    material: Material,
) -> (Vec<[VertexData; 3]>, Matrix4<f32>, Material) {
    let transform = Matrix4::from_translation(pos)
        * Matrix4::from_angle_y(Rad(rotation.y))
        * Matrix4::from_angle_x(Rad(rotation.x))
        * Matrix4::from_angle_z(Rad(rotation.z))
        * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

    (cube_model().to_vec(), transform, material)
}

// portal surfaces are not rotated in this example
fn portal_surface(viewport_size: Vector2<i32>, pos: Vector3<f32>, size: Vector2<f32>) -> Portal {
    let transform =
        Matrix4::from_translation(pos) * Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);

    Portal {
        transform,
        render_target: Image2d::new(
            RgbaU8::new(vec4(108, 182, 204, 0xFF)),
            viewport_size.x as u32,
            viewport_size.y as u32,
        ),
        depth_buffer: Image2d::new(
            DepthF32::new(1.0),
            viewport_size.x as u32,
            viewport_size.y as u32,
        ),
    }
}

pub struct Portal {
    pub transform: Matrix4<f32>,
    pub render_target: Image2d<RgbaU8>,
    pub depth_buffer: Image2d<DepthF32>,
}

pub struct Scene {
    pub objects: Vec<(Vec<[VertexData; 3]>, Matrix4<f32>, Material)>,
    pub portal0: Portal,
    pub portal1: Portal,
}

pub fn scene(viewport_size: Vector2<i32>) -> Scene {
    let mut room = rectangular_prism(
        vec3(0.0, 0.0, 0.0),
        Vector3::zero(),
        vec3(-5.0, -5.0, -10.5),
        Material::MATERIAL0,
    );

    // need to flip the normals since the scale is negative
    for tri in &mut room.0 {
        tri[0].flip_normal();
        tri[1].flip_normal();
        tri[2].flip_normal();
    }

    let box1 = rectangular_prism(
        vec3(0.0, 0.0, 2.0),
        vec3(0.5, 0.8, -1.1),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box2 = rectangular_prism(
        vec3(-1.5, 1.0, 3.0),
        vec3(-0.5, -0.4, -0.6),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box3 = rectangular_prism(
        vec3(1.5, 2.0, 3.0),
        vec3(0.9, 1.5, 0.4),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box4 = rectangular_prism(
        vec3(-2.0, -2.0, 4.0),
        vec3(1.2, 1.8, 0.8),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box5 = rectangular_prism(
        vec3(2.0, -0.8, 4.0),
        vec3(1.2, 1.8, 0.4),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box6 = rectangular_prism(
        vec3(0.0, 0.0, -4.0),
        vec3(0.5, 0.8, -1.1),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box7 = rectangular_prism(
        vec3(1.5, 1.0, -3.0),
        vec3(-0.5, -0.4, -0.6),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box8 = rectangular_prism(
        vec3(1.5, -2.0, -3.0),
        vec3(0.9, 1.5, 0.4),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box9 = rectangular_prism(
        vec3(-2.0, 2.0, -4.0),
        vec3(1.2, 1.8, 0.8),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let box10 = rectangular_prism(
        vec3(-2.0, -0.8, -3.5),
        vec3(1.2, 1.8, 0.4),
        vec3(0.8, 0.8, 0.8),
        Material::MATERIAL0,
    );

    let floor = rectangular_prism(
        vec3(12.0, -2.0, 0.0),
        Vector3::zero(),
        vec3(5.0, 1.0, 15.0),
        Material::MATERIAL1,
    );

    let box11 = rectangular_prism(
        vec3(11.1, -1.0, -4.0),
        vec3(0.0, -0.8, 0.0),
        vec3(1.0, 1.0, 1.0),
        Material::MATERIAL1,
    );

    let box12 = rectangular_prism(
        vec3(11.9, -1.0, -6.0),
        vec3(0.0, -0.4, 0.0),
        vec3(1.0, 1.0, 1.0),
        Material::MATERIAL1,
    );

    let box13 = rectangular_prism(
        vec3(13.3, -1.0, -5.0),
        vec3(0.0, 0.7, 0.0),
        vec3(1.0, 1.0, 1.0),
        Material::MATERIAL1,
    );

    Scene {
        objects: vec![
            room, box1, box2, box3, box4, box5, box6, box7, box8, box9, box10, floor, box11, box12,
            box13,
        ],
        portal0: portal_surface(viewport_size, Vector3::zero(), vec2(5.0, 5.0)),
        portal1: portal_surface(viewport_size, vec3(12.0, 1.5, 0.0), vec2(5.0, 5.0)),
    }
}
