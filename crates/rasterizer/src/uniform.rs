pub trait Uniform: Sync {}

impl<T: Sync> Uniform for T {}

const UNIT_TYPE_BUF: [(); 1] = [()];

pub fn unit_type_buf() -> &'static [()] {
    &UNIT_TYPE_BUF
}

pub struct Uniforms<'a, U0: Uniform, U1: Uniform, U2: Uniform, U3: Uniform> {
    buffer0: &'a [U0],
    buffer1: &'a [U1],
    buffer2: &'a [U2],
    buffer3: &'a [U3],
}

impl<'a, U0: Uniform, U1: Uniform, U2: Uniform, U3: Uniform> Uniforms<'a, U0, U1, U2, U3> {
    pub fn new(buffer0: &'a [U0], buffer1: &'a [U1], buffer2: &'a [U2], buffer3: &'a [U3]) -> Self {
        Self {
            buffer0,
            buffer1,
            buffer2,
            buffer3,
        }
    }

    pub fn get(&self, indices: [u32; 4]) -> (&U0, &U1, &U2, &U3) {
        (
            &self.buffer0[indices[0] as usize],
            &self.buffer1[indices[1] as usize],
            &self.buffer2[indices[2] as usize],
            &self.buffer3[indices[3] as usize],
        )
    }
}
