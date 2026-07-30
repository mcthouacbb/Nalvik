use cgmath::{Vector2, Vector3};

fn cross2<T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(
    a: Vector2<T>,
    b: Vector2<T>,
) -> T {
    a.x * b.y - a.y * b.x
}

// v should be in [-1, 1]^2
fn to_viewport(v: Vector2<f32>, viewport_size: Vector2<i32>) -> Vector2<f32> {
    Vector2::new(
        (v.x * 0.5 + 0.5) * viewport_size.x as f32,
        (0.5 - v.y * 0.5) * viewport_size.y as f32,
    )
}

struct EdgeFn {
    pub dx: f32,
    pub dy: f32,
    pub c: f32,
}

impl EdgeFn {
    fn from_edge(v0: Vector2<f32>, v1: Vector2<f32>) -> Self {
        let dx = v1.y - v0.y;
        let dy = v0.x - v1.x;
        let neg_c = v0.x * dx + v0.y * dy;
        Self { dx, dy, c: -neg_c }
    }

    fn evaluate(&self, pos: Vector2<f32>) -> f32 {
        self.dx * pos.x + self.dy * pos.y + self.c
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
    // AB x CA
    // let tri_area = cross2(v1 - v0, v0 - v2);

    let edge0 = EdgeFn::from_edge(v1, v2);
    let edge1 = EdgeFn::from_edge(v2, v0);
    let edge2 = EdgeFn::from_edge(v0, v1);

    let norm0 = 1.0 / edge0.evaluate(v0);
    let norm1 = 1.0 / edge1.evaluate(v1);
    let norm2 = 1.0 / edge2.evaluate(v2);

    let min_x = v0.x.min(v1.x).min(v2.x);
    let max_x = v0.x.max(v1.x).max(v2.x);
    let min_y = v0.y.min(v1.y).min(v2.y);
    let max_y = v0.y.max(v1.y).max(v2.y);

    let mut y = (min_y - 0.5).ceil() + 0.5;
    while y < max_y {
        assert!(y.fract() == 0.5);

        let mut x = (min_x - 0.5).ceil() + 0.5;
        while x < max_x {
            assert!(x.fract() == 0.5);

            let p = Vector2::new(x, y);
            let e0 = edge0.evaluate(p);
            let e1 = edge1.evaluate(p);
            let e2 = edge2.evaluate(p);

            if e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0 {
                let barycentric = Vector3::new(e0 * norm0, e1 * norm1, e2 * norm2);
                pixel_fn(x as u32, y as u32, barycentric);
            }

            x += 1.0;
        }

        y += 1.0;
    }
}
