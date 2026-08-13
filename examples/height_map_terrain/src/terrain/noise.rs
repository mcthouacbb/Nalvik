use cgmath::{Vector2, Vector3, vec2, vec3};
use noise::NoiseFn;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Biome {
    Plains,
    Mountain,
    Rocky,
    Ocean,
}

impl Biome {
    pub fn get_color(&self, vertex: Vector3<f32>) -> Vector3<f32> {
        match self {
            Self::Ocean => {
                if vertex.y < 0.3 {
                    vec3(0.612, 0.604, 0.584)
                } else {
                    vec3(0.831, 0.761, 0.325)
                }
            }
            Self::Plains => vec3(0.086, 0.651, 0.357),
            Self::Rocky => vec3(0.659, 0.678, 0.71),
            Self::Mountain => {
                if vertex.y < 22.5 {
                    vec3(0.459, 0.329, 0.082)
                } else {
                    vec3(0.7, 0.8, 0.9)
                }
            }
        }
    }
}

pub struct Noise {
    a_noise: noise::Simplex,
    m_noise: noise::Simplex,
    r_noise: noise::Simplex,
    b_noise: noise::Simplex,
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

fn smooth_clamp(x: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    smooth_min(smooth_max(x, min), max).clamp(min, max)
}

impl Noise {
    pub fn new() -> Self {
        Self {
            a_noise: noise::Simplex::new(0x2A8D2F39),
            m_noise: noise::Simplex::new(0x473829),
            r_noise: noise::Simplex::new(0x384B729),
            b_noise: noise::Simplex::new(0x38DD390B),
        }
    }

    pub fn land(&self, pos: Vector2<f32>) -> f32 {
        let noise_pos = pos * 0.004;
        smooth_clamp(
            self.a_noise.get([noise_pos.x as f64, noise_pos.y as f64]) as f32 * 4.0 + 0.2,
            -0.5,
            1.0,
        )
    }

    pub fn mountainous(&self, pos: Vector2<f32>) -> f32 {
        let noise_pos = pos * 0.01;
        smooth_clamp(
            self.m_noise.get([noise_pos.x as f64, noise_pos.y as f64]) as f32 * 2.0 - 0.4,
            0.0,
            1.0,
        )
    }

    pub fn roughness(&self, pos: Vector2<f32>) -> f32 {
        let noise_pos = pos * 0.01;
        smooth_clamp(
            self.r_noise.get([noise_pos.x as f64, noise_pos.y as f64]) as f32 * 1.7 - 0.2,
            0.0,
            1.0,
        )
    }

    pub fn base_noise(&self, pos: Vector2<f32>) -> (f32, f32) {
        let mut noise_pos = pos * 0.04;
        let base = self.b_noise.get([noise_pos.x as f64, noise_pos.y as f64]) as f32 * 0.5 + 0.5;

        let mut fbm = 0.0;
        let mut scale = 1.94;
        for _ in 0..3 {
            noise_pos = 3.0 * (noise_pos - vec2(107.234, -86.3847));
            scale *= 0.35;
            fbm += self.b_noise.get([noise_pos.x as f64, noise_pos.y as f64]) as f32 * scale;
        }

        (base, fbm * 0.5 + 0.5)
    }

    pub fn get(&self, pos: Vector2<f32>) -> (f32, Biome) {
        let actual_pos = pos;

        let land = self.land(actual_pos);
        let mut mountainous = self.mountainous(actual_pos).powf(2.0);
        let mut roughness = self.roughness(actual_pos);
        let (base, fbm) = self.base_noise(actual_pos);

        mountainous *= land.max(0.0);
        roughness *= land.max(0.0);

        let height = (land * 3.0 + mountainous * 30.0) * base
            + (mountainous * 3.0 + roughness) * land.max(0.0) * 4.0 * fbm;

        let biome = if land < 0.2 && height < 1.0 {
            Biome::Ocean
        } else if mountainous > 0.5 {
            Biome::Mountain
        } else if mountainous * 3.0 + roughness > 0.5 {
            Biome::Rocky
        } else {
            Biome::Plains
        };

        (land * 3.0 + height, biome)
    }
}
