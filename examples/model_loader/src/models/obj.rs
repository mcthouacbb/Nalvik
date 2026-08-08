use std::{fmt, path::Path};

use cgmath::{vec2, vec3, vec4};

use crate::{material::Material, models::VertexData};

pub fn load_obj_model(
    path: impl AsRef<Path> + fmt::Debug,
) -> Vec<(Vec<[VertexData; 3]>, Material)> {
    let (obj_models, obj_materials) = match tobj::load_obj(&path, &tobj::GPU_LOAD_OPTIONS) {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "Error! Could not load obj file '{}': {}",
                path.as_ref().display(),
                err.to_string()
            );
            return vec![(Vec::new(), Material::debug_material())];
        }
    };

    let mut materials = Vec::new();
    match obj_materials {
        Ok(obj_materials) => {
            for obj_material in obj_materials {
                let mut material = None;

                if let Some(texture_file) = obj_material.diffuse_texture {
                    material = match Material::try_load_from_file(
                        path.as_ref().parent().unwrap().join(&texture_file),
                    ) {
                        Ok(material) => Some(material),
                        Err(err) => {
                            println!(
                                "Error! Could not load texture file '{}': {}",
                                texture_file,
                                err.to_string()
                            );
                            None
                        }
                    }
                }

                if material.is_none() {
                    if let Some(color) = obj_material.diffuse {
                        material = Some(Material::solid_color(vec4(
                            color[0], color[1], color[2], 1.0,
                        )));
                    } else {
                        println!("Falling back to debug material");
                        material = Some(Material::debug_material());
                    }
                }

                assert!(material.is_some());
                materials.push(material.unwrap());
            }
        }
        Err(err) => {
            eprintln!(
                "Error! Could not material files: {}. Falling back to default material",
                err
            );
        }
    }

    let mut result = Vec::new();

    for model in &obj_models {
        let mesh = &model.mesh;

        let mut triangles = Vec::new();

        for indices in mesh.indices.chunks_exact(3) {
            let mut triangle = [VertexData::default(); 3];
            for (vertex_index, &index) in indices.iter().enumerate() {
                let i = index as usize;
                let pos = vec3(
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                );
                let normal = vec3(
                    mesh.normals[3 * i],
                    mesh.normals[3 * i + 1],
                    mesh.normals[3 * i + 2],
                );
                let uv = vec2(mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1]);
                triangle[vertex_index] = VertexData::new(pos, uv, normal);
            }
            triangles.push(triangle);
        }

        let material = if let Some(id) = mesh.material_id
            && id < materials.len()
        {
            materials[id].clone()
        } else {
            Material::debug_material()
        };

        result.push((triangles, material));
    }

    result
}
