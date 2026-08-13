use cgmath::{Vector3, Vector4, prelude::*, vec3};

#[derive(Clone, Copy, PartialEq)]
pub enum BlendOp {
    Add,
    Sub,
    ReverseSub,
}

impl BlendOp {
    pub fn apply_color(
        &self,
        blended_src: Vector3<f32>,
        blended_dst: Vector3<f32>,
    ) -> Vector3<f32> {
        match self {
            BlendOp::Add => blended_src + blended_dst,
            BlendOp::Sub => blended_src - blended_dst,
            BlendOp::ReverseSub => blended_dst - blended_src,
        }
    }

    pub fn apply_alpha(&self, blended_src: f32, blended_dst: f32) -> f32 {
        match self {
            BlendOp::Add => blended_src + blended_dst,
            BlendOp::Sub => blended_src - blended_dst,
            BlendOp::ReverseSub => blended_dst - blended_src,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
}

impl BlendFactor {
    pub fn compute_blended_src_dst<const CLAMP: bool>(
        src_color_factor: BlendFactor,
        src_alpha_factor: BlendFactor,
        dst_color_factor: BlendFactor,
        dst_alpha_factor: BlendFactor,
        mut src: Vector4<f32>,
        mut dst: Vector4<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32, f32) {
        if CLAMP {
            src = src.map(|x| x.clamp(0.0, 1.0));
            dst = dst.map(|x| x.clamp(0.0, 1.0));
        }

        let blended_src_color = match src_color_factor {
            Self::Zero => Vector3::zero(),
            Self::One => src.xyz(),
            Self::SrcColor => src.xyz().mul_element_wise(src.xyz()),
            Self::OneMinusSrcColor => src.xyz().mul_element_wise(vec3(1.0, 1.0, 1.0) - src.xyz()),
            Self::DstColor => src.xyz().mul_element_wise(dst.xyz()),
            Self::OneMinusDstColor => src.xyz().mul_element_wise(vec3(1.0, 1.0, 1.0) - dst.xyz()),
            Self::SrcAlpha => src.xyz() * src.w,
            Self::OneMinusSrcAlpha => src.xyz() * (1.0 - src.w),
            Self::DstAlpha => src.xyz() * dst.w,
            Self::OneMinusDstAlpha => src.xyz() * (1.0 - dst.w),
        };

        let blended_dst_color = match dst_color_factor {
            Self::Zero => Vector3::zero(),
            Self::One => dst.xyz(),
            Self::SrcColor => dst.xyz().mul_element_wise(src.xyz()),
            Self::OneMinusSrcColor => dst.xyz().mul_element_wise(vec3(1.0, 1.0, 1.0) - src.xyz()),
            Self::DstColor => dst.xyz().mul_element_wise(dst.xyz()),
            Self::OneMinusDstColor => dst.xyz().mul_element_wise(vec3(1.0, 1.0, 1.0) - dst.xyz()),
            Self::SrcAlpha => dst.xyz() * src.w,
            Self::OneMinusSrcAlpha => dst.xyz() * (1.0 - src.w),
            Self::DstAlpha => dst.xyz() * dst.w,
            Self::OneMinusDstAlpha => dst.xyz() * (1.0 - dst.w),
        };

        let blended_src_alpha = match src_alpha_factor {
            Self::Zero => 0.0,
            Self::One => src.w,
            Self::SrcColor | Self::SrcAlpha => src.w * src.w,
            Self::OneMinusSrcColor | Self::OneMinusSrcAlpha => src.w * (1.0 - src.w),
            Self::DstColor | Self::DstAlpha => src.w * dst.w,
            Self::OneMinusDstColor | Self::OneMinusDstAlpha => src.w * (1.0 - dst.w),
        };

        let blended_dst_alpha = match dst_alpha_factor {
            Self::Zero => 0.0,
            Self::One => dst.w,
            Self::SrcColor | Self::SrcAlpha => dst.w * src.w,
            Self::OneMinusSrcColor | Self::OneMinusSrcAlpha => dst.w * (1.0 - src.w),
            Self::DstColor | Self::DstAlpha => dst.w * dst.w,
            Self::OneMinusDstColor | Self::OneMinusDstAlpha => dst.w * (1.0 - dst.w),
        };

        (
            blended_src_color,
            blended_dst_color,
            blended_src_alpha,
            blended_dst_alpha,
        )
    }
}

pub struct BlendState {
    color_blend_op: BlendOp,
    alpha_blend_op: BlendOp,
    src_color_blend_factor: BlendFactor,
    dst_color_blend_factor: BlendFactor,
    src_alpha_blend_factor: BlendFactor,
    dst_alpha_blend_factor: BlendFactor,
}

impl BlendState {
    pub fn blend<const CLAMP: bool>(&self, src: Vector4<f32>, dst: Vector4<f32>) -> Vector4<f32> {
        let (blended_src_color, blended_dst_color, blended_src_alpha, blended_dst_alpha) =
            BlendFactor::compute_blended_src_dst::<CLAMP>(
                self.src_color_blend_factor,
                self.src_alpha_blend_factor,
                self.dst_color_blend_factor,
                self.dst_alpha_blend_factor,
                src,
                dst,
            );

        self.color_blend_op
            .apply_color(blended_src_color, blended_dst_color)
            .extend(
                self.alpha_blend_op
                    .apply_alpha(blended_src_alpha, blended_dst_alpha),
            )
    }
}
