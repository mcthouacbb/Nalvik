use cgmath::{Rad, Vector2, vec2};
use noise::NoiseFn;

pub struct Noise {
    s_noise0: noise::Simplex,
    s_noise1: noise::Simplex,
    h_noise0: noise::Simplex,
}

fn smooth_min(a: f32, b: f32) -> f32 {
    const K: f32 = 0.05;
    let r = (-a / K).exp2() + (-b / K).exp2();
    -K * r.log2()
    // a.min(b)
}

fn smooth_max(a: f32, b: f32) -> f32 {
    -smooth_min(-a, -b)
    // a.max(b)
}

impl Noise {
    pub fn new() -> Self {
        Self {
            s_noise0: noise::Simplex::new(0x2A8D2F39),
            s_noise1: noise::Simplex::new(0x473829),
            h_noise0: noise::Simplex::new(0x38DD390B),
        }
    }

    pub fn get(&self, pos: Vector2<f32>) -> f32 {
        let actual_pos = pos * 0.7;
        let steepness0 = smooth_min(
            smooth_max(
                self.s_noise0
                    .get([actual_pos.x as f64 * 0.03, actual_pos.y as f64 * 0.03])
                    as f32
                    * 2.0
                    + 0.5,
                0.0,
            ),
            1.0,
        );
        let steepness1 = smooth_min(
            smooth_max(
                self.s_noise1
                    .get([actual_pos.x as f64 * 0.03, actual_pos.y as f64 * 0.03])
                    as f32
                    * 2.0
                    + 0.5,
                0.0,
            ),
            1.0,
        );

        let steepness = steepness0 * steepness1;

        let matrix = cgmath::Matrix2::from_angle(Rad(0.43));
        let noise_pos0 = actual_pos * 0.08;
        let noise_pos1 = 2.5 * (matrix * (noise_pos0 + vec2(-106.34, 89.225)));
        let base_height = self
            .h_noise0
            .get([noise_pos0.x as f64, noise_pos0.y as f64]) as f32
            * 8.0
            + self
                .h_noise0
                .get([noise_pos1.x as f64, noise_pos1.y as f64]) as f32
                * 1.6
            + 9.6;

        (base_height + 3.0) * steepness * 0.85
    }
}
