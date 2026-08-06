use arrayvec::ArrayVec;
use cgmath::{Vector2, Vector4, prelude::*, vec4};

use crate::pipeline::{VertexOutput, vertex_to_fragment::VertexToFragment};

pub const BUF_SIZE: usize = 32 * 3;

/*
 * clip space bounds
 * -w <= x <= w
 * -w <= y <= w
 *  0 <= z <= w
 *
 * technically w > 0 is also a bound, but I'll just assume
 * it's true given the other 3 bounds even though it
 * isn't in general
 *
 * guard band clipping is used, so x and y can appear outside the specified range
 * but they will not be so large as to cause overflows during rasterization
 */

pub fn clip_triangle<Vo: VertexToFragment>(
    v0: &VertexOutput<Vo>,
    v1: &VertexOutput<Vo>,
    v2: &VertexOutput<Vo>,
    out_buf: &mut ArrayVec<VertexOutput<Vo>, BUF_SIZE>,
    viewport_size: Vector2<i32>,
) {
    debug_assert!(out_buf.len() == 0);

    let mut tmp_buf = ArrayVec::<VertexOutput<Vo>, BUF_SIZE>::new();
    let mut front_buf = out_buf;
    let mut back_buf = &mut tmp_buf;

    // z >= 0
    clip_against_plane(v0, v1, v2, vec4(0.0, 0.0, 1.0, 0.0), front_buf);
    debug_assert!(front_buf.len() % 3 == 0);

    if front_buf.len() == 0 {
        return;
    }

    const TWO_POW_17: f32 = 131072.0;

    // guard band clipping
    let coeff_neg_x = -TWO_POW_17 / viewport_size.x as f32 - 1.0;
    let coeff_pos_x = TWO_POW_17 / viewport_size.x as f32 - 1.0;
    let coeff_neg_y = -TWO_POW_17 / viewport_size.y as f32 + 1.0;
    let coeff_pos_y = TWO_POW_17 / viewport_size.y as f32 + 1.0;

    std::mem::swap(&mut front_buf, &mut back_buf);
    for vertices in back_buf.chunks_exact(3) {
        if outside_xy_clip_planes(&vertices[0], &vertices[1], &vertices[2]) {
            continue;
        }

        clip_against_plane(
            &vertices[0],
            &vertices[1],
            &vertices[2],
            vec4(1.0, 0.0, 0.0, -coeff_neg_x),
            front_buf,
        );
    }

    std::mem::swap(&mut front_buf, &mut back_buf);
    front_buf.clear();
    for vertices in back_buf.chunks_exact(3) {
        clip_against_plane(
            &vertices[0],
            &vertices[1],
            &vertices[2],
            vec4(-1.0, 0.0, 0.0, coeff_pos_x),
            &mut front_buf,
        );
    }

    std::mem::swap(&mut front_buf, &mut back_buf);
    front_buf.clear();
    for vertices in back_buf.chunks_exact(3) {
        clip_against_plane(
            &vertices[0],
            &vertices[1],
            &vertices[2],
            vec4(0.0, 1.0, 0.0, -coeff_neg_y),
            &mut front_buf,
        );
    }

    std::mem::swap(&mut front_buf, &mut back_buf);
    front_buf.clear();
    for vertices in back_buf.chunks_exact(3) {
        clip_against_plane(
            &vertices[0],
            &vertices[1],
            &vertices[2],
            vec4(0.0, -1.0, 0.0, coeff_pos_y),
            &mut front_buf,
        );
    }
}

// can return false negatives (but not false positives)
pub fn outside_xy_clip_planes<Vo: VertexToFragment>(
    v0: &VertexOutput<Vo>,
    v1: &VertexOutput<Vo>,
    v2: &VertexOutput<Vo>,
) -> bool {
    if v0.position.x < -v0.position.w
        && v1.position.x < -v1.position.w
        && v2.position.x < -v2.position.w
    {
        return true;
    }

    if v0.position.y < -v0.position.w
        && v1.position.y < -v1.position.w
        && v2.position.y < -v2.position.w
    {
        return true;
    }

    if v0.position.x > v0.position.w
        && v1.position.x > v1.position.w
        && v2.position.x > v2.position.w
    {
        return true;
    }

    if v0.position.y > v0.position.w
        && v1.position.y > v1.position.w
        && v2.position.y > v2.position.w
    {
        return true;
    }

    false
}

pub fn clip_against_plane<Vo: VertexToFragment>(
    v0: &VertexOutput<Vo>,
    v1: &VertexOutput<Vo>,
    v2: &VertexOutput<Vo>,
    plane: Vector4<f32>,
    out_buf: &mut ArrayVec<VertexOutput<Vo>, BUF_SIZE>,
) {
    // clip the triangle to ensure dot(plane, p) >= 0 for all points on the output triangle(s)
    let res0 = v0.position.dot(plane);
    let res1 = v1.position.dot(plane);
    let res2 = v2.position.dot(plane);

    let outside = (res0 < 0.0) as i32 + (res1 < 0.0) as i32 + (res2 < 0.0) as i32;
    match outside {
        0 => {
            out_buf.push(*v0);
            out_buf.push(*v1);
            out_buf.push(*v2);
        }
        1 => {
            let (outside, inside0, inside1) = {
                if res0 < 0.0 {
                    ((v0, res0), (v1, res1), (v2, res2))
                } else if res1 < 0.0 {
                    ((v1, res1), (v2, res2), (v0, res0))
                } else {
                    ((v2, res2), (v0, res0), (v1, res1))
                }
            };

            let v2 = VertexOutput::<Vo>::interpolate2(
                outside.0,
                inside1.0,
                -outside.1 / (inside1.1 - outside.1),
            );

            let v3 = VertexOutput::<Vo>::interpolate2(
                outside.0,
                inside0.0,
                -outside.1 / (inside0.1 - outside.1),
            );

            out_buf.push(*inside0.0);
            out_buf.push(*inside1.0);
            out_buf.push(v2);
            out_buf.push(v2);
            out_buf.push(v3);
            out_buf.push(*inside0.0);
        }
        2 => {
            let (inside, outside0, outside1) = {
                if res0 >= 0.0 {
                    ((v0, res0), (v1, res1), (v2, res2))
                } else if res1 >= 0.0 {
                    ((v1, res1), (v2, res2), (v0, res0))
                } else {
                    ((v2, res2), (v0, res0), (v1, res1))
                }
            };

            let v1 = VertexOutput::<Vo>::interpolate2(
                outside0.0,
                inside.0,
                -outside0.1 / (inside.1 - outside0.1),
            );

            let v2 = VertexOutput::<Vo>::interpolate2(
                outside1.0,
                inside.0,
                -outside1.1 / (inside.1 - outside1.1),
            );

            out_buf.push(*inside.0);
            out_buf.push(v1);
            out_buf.push(v2);
        }
        3 => (),
        _ => unreachable!(),
    }
}
