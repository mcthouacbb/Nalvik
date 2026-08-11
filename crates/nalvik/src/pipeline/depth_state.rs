use crate::image::{
    format::DepthFormat,
    view::{Image2dView, Image2dViewMut},
};

#[derive(Clone, Copy)]
pub enum DepthTest {
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
}

impl DepthTest {
    pub fn compare<D: DepthFormat>(&self, old: &D, new: &D) -> bool {
        match self {
            Self::Less => D::compare_less(new, old),
            Self::Greater => D::compare_greater(new, old),
            Self::LessEqual => D::compare_less_equal(new, old),
            Self::GreaterEqual => D::compare_greater_equal(new, old),
            Self::Equal => D::compare_equal(new, old),
        }
    }
}

pub enum DepthState<'a, D: DepthFormat> {
    CompareOnly(Image2dView<'a, D>, DepthTest),
    WriteOnly(Image2dViewMut<'a, D>),
    CompareAndWrite(Image2dViewMut<'a, D>, DepthTest),
    None,
}
