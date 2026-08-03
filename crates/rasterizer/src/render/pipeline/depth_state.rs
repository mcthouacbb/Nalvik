use crate::render::image::{
    format::DepthFormat,
    view::{ImageView, ImageViewMut},
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
    CompareOnly(ImageView<'a, D>, DepthTest),
    WriteOnly(ImageViewMut<'a, D>),
    CompareAndWrite(ImageViewMut<'a, D>, DepthTest),
    None,
}

impl<'a, D: DepthFormat> DepthState<'a, D> {
    pub fn keep_fragment(&mut self, x: u32, y: u32, depth: f32) -> bool {
        match self {
            Self::CompareOnly(image_view, depth_test) => {
                depth_test.compare(image_view.get(x, y), &D::from(depth))
            }
            Self::WriteOnly(image_view) => {
                *image_view.get_mut(x, y) = D::from(depth);
                true
            }
            Self::CompareAndWrite(image_view, depth_test) => {
                let old = image_view.get_mut(x, y);
                let new = D::from(depth);
                if depth_test.compare(old, &new) {
                    *old = new;
                    true
                } else {
                    false
                }
            }
            Self::None => true,
        }
    }
}
