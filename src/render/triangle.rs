use cgmath::{Vector2, Vector3};

const ONE: i32 = 256;
const HALF: i32 = ONE / 2;

const _: () = assert!(ONE & (ONE - 1) == 0 && ONE > 1);

fn fixed_floor(x: i32) -> i32 {
    x & !(ONE - 1)
}

fn fixed_ceil(x: i32) -> i32 {
    fixed_floor(x + ONE - 1)
}

// v should be in [-1, 1]^2
fn to_viewport(v: Vector2<f32>, viewport_size: Vector2<i32>) -> Vector2<i32> {
    Vector2::new(
        ((v.x * 0.5 + 0.5) * (viewport_size.x * ONE) as f32).round() as i32,
        ((0.5 - v.y * 0.5) * (viewport_size.y * ONE) as f32).round() as i32,
    )
}

struct EdgeFn {
    pub dx: i64,
    pub dy: i64,
    pub c: i64,
}

impl EdgeFn {
    fn from_edge(v0: Vector2<i32>, v1: Vector2<i32>) -> Self {
        let dx = (v1.y - v0.y) as i64;
        let dy = (v0.x - v1.x) as i64;
        let c = -v0.x as i64 * dx as i64 - v0.y as i64 * dy as i64;
        Self { dx, dy, c }
    }

    fn evaluate(&self, pos: Vector2<i32>) -> i64 {
        self.dx * pos.x as i64 + self.dy * pos.y as i64 + self.c
    }
}

// must be CCW winding order
pub fn render_triangle(
    v0: Vector2<f32>,
    v1: Vector2<f32>,
    v2: Vector2<f32>,
    viewport_size: Vector2<i32>,
    mut pixel_fn: impl FnMut(u32, u32, Vector3<f32>),
) {
    let v0 = to_viewport(v0, viewport_size);
    let v1 = to_viewport(v1, viewport_size);
    let v2 = to_viewport(v2, viewport_size);

    let edge0 = EdgeFn::from_edge(v1, v2);
    let edge1 = EdgeFn::from_edge(v2, v0);
    let edge2 = EdgeFn::from_edge(v0, v1);

    let norm0 = 1.0 / edge0.evaluate(v0) as f32;
    let norm1 = 1.0 / edge1.evaluate(v1) as f32;
    let norm2 = 1.0 / edge2.evaluate(v2) as f32;

    let min_x = v0.x.min(v1.x).min(v2.x);
    let max_x = v0.x.max(v1.x).max(v2.x);
    let min_y = v0.y.min(v1.y).min(v2.y);
    let max_y = v0.y.max(v1.y).max(v2.y);

    let mut y = fixed_ceil(min_y - HALF) + HALF;
    while y < max_y {
        debug_assert!(y % ONE == HALF);

        let mut x = fixed_ceil(min_x - HALF) + HALF;
        while x < max_x {
            debug_assert!(x % ONE == HALF);

            let p = Vector2::new(x, y);
            let e0 = edge0.evaluate(p);
            let e1 = edge1.evaluate(p);
            let e2 = edge2.evaluate(p);

            // TODO: top left rule
            if e0 >= 0 && e1 >= 0 && e2 >= 0 {
                let barycentric =
                    Vector3::new(e0 as f32 * norm0, e1 as f32 * norm1, e2 as f32 * norm2);
                pixel_fn((x / ONE) as u32, (y / ONE) as u32, barycentric);
            }

            x += ONE;
        }

        y += ONE;
    }
}
