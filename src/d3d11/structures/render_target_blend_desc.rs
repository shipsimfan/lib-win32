use crate::{
    d3d11::{D3D11_BLEND, D3D11_BLEND_OP, D3D11_COLOR_WRITE_ENABLE},
    BOOL, FALSE, UINT8,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_BLEND_DESC;

/// Describes the blend state for a render target.
///
/// # Remarks
/// You specify an array of [`D3D11_RENDER_TARGET_BLEND_DESC`] structures in the `render_target`
/// member of the [`D3D11_BLEND_DESC`] structure to describe the blend states for render targets;
/// you can bind up to eight render targets to the output-merger stage at one time.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    /// Enable (or disable) blending.
    pub blend_enable: BOOL,

    /// This blend option specifies the operation to perform on the RGB value that the pixel shader
    /// outputs. The `blend_op` member defines how to combine the `src_blend` and `dest_blend`
    /// operations.
    pub src_blend: D3D11_BLEND,

    /// This blend option specifies the operation to perform on the current RGB value in the render
    /// target. The `blend_op` member defines how to combine the `src_blend` and `dest_blend`
    /// operations.
    pub dest_blend: D3D11_BLEND,

    /// This blend operation defines how to combine the `src_blend` and `dest_blend` operations.
    pub blend_op: D3D11_BLEND_OP,

    /// This blend option specifies the operation to perform on the alpha value that the pixel
    /// shader outputs. Blend options that end in `_COLOR` are not allowed. The `blend_op_alpha`
    /// member defines how to combine the `src_blend_alpha` and `dest_blend_alpha` operations.
    pub src_blend_alpha: D3D11_BLEND,

    /// This blend option specifies the operation to perform on the current alpha value in the
    /// render target. Blend options that end in `_COLOR` are not allowed. The `blend_op_alpha`
    /// member defines how to combine the `src_blend_alpha` and `dest_blend_alpha` operations.
    pub dest_blend_alpha: D3D11_BLEND,

    /// This blend operation defines how to combine the `src_blend_alpha` and `dest_blend_alpha`
    /// operations.
    pub blend_op_alpha: D3D11_BLEND_OP,

    /// A write mask.
    pub render_target_write_mask: UINT8,
}

impl Default for D3D11_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        D3D11_RENDER_TARGET_BLEND_DESC {
            blend_enable: FALSE,
            src_blend: D3D11_BLEND::One,
            dest_blend: D3D11_BLEND::Zero,
            blend_op: D3D11_BLEND_OP::Add,
            src_blend_alpha: D3D11_BLEND::One,
            dest_blend_alpha: D3D11_BLEND::Zero,
            blend_op_alpha: D3D11_BLEND_OP::Add,
            render_target_write_mask: D3D11_COLOR_WRITE_ENABLE::All as _,
        }
    }
}
