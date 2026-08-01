pub trait Uniforms: Copy {}

impl Uniforms for () {}
impl<T> Uniforms for &T {}
impl<T, U> Uniforms for (&T, &U) {}
impl<T, U, V> Uniforms for (&T, &U, &V) {}
impl<T, U, V, W> Uniforms for (&T, &U, &V, &W) {}
