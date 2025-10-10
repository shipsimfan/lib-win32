use crate::{
    d3d11::{D3D11_CULL_MODE, D3D11_FILL_MODE},
    BOOL, FALSE, FLOAT, INT, TRUE, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11DeviceContext, d3d11_1::ID3D11Device1};

/// Describes rasterizer state.
///
/// # Remarks
/// Rasterizer state defines the behavior of the rasterizer stage. To create a rasterizer-state
/// object, call [`ID3D11Device1::create_rasterizer_state1`]. To set rasterizer state, call
/// [`ID3D11DeviceContext::rs_set_state`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_RASTERIZER_DESC1 {
    /// Determines the fill mode to use when rendering.
    pub fill_mode: D3D11_FILL_MODE,

    /// Indicates that triangles facing the specified direction are not drawn.
    pub cull_mode: D3D11_CULL_MODE,

    /// Specifies whether a triangle is front- or back-facing. If [`TRUE`], a triangle will be
    /// considered front-facing if its vertices are counter-clockwise on the render target and
    /// considered back-facing if they are clockwise. If [`FALSE`], the opposite is true.
    pub front_counter_clockwise: BOOL,

    /// Depth value added to a given pixel.
    pub depth_bias: INT,

    /// Maximum depth bias of a pixel.
    pub depth_bias_clamp: FLOAT,

    /// Scalar on a given pixel's slope.
    pub slope_scaled_depth_bias: FLOAT,

    /// Specifies whether to enable clipping based on distance.
    ///
    /// The hardware always performs x and y clipping of rasterized coordinates. When
    /// `depth_clip_enable` is set to the default–[`TRUE`], the hardware also clips the z value.
    ///
    /// When you set `depth_clip_enable` to [`FALSE`], the hardware skips the z clipping. However,
    /// the hardware still performs the "0 < w" clipping. When z clipping is disabled, improper
    /// depth ordering at the pixel level might result. However, when z clipping is disabled,
    /// stencil shadow implementations are simplified. In other words, you can avoid complex
    /// special-case handling for geometry that goes beyond the back clipping plane.
    pub depth_clip_enable: BOOL,

    /// Specifies whether to enable scissor-rectangle culling. All pixels outside an active scissor
    /// rectangle are culled.
    pub scissor_enable: BOOL,

    /// Specifies whether to use the quadrilateral or alpha line anti-aliasing algorithm on
    /// multisample antialiasing (MSAA) render targets. Set to [`TRUE`] to use the quadrilateral
    /// line anti-aliasing algorithm and to [`FALSE`] to use the alpha line anti-aliasing
    /// algorithm.
    pub multisample_enable: BOOL,

    /// Specifies whether to enable line antialiasing; only applies if doing line drawing and
    /// `multisample_enable` is [`FALSE`].
    pub antialiased_line_enable: BOOL,

    /// The sample count that is forced while UAV rendering or rasterizing. Valid values are 0, 1,
    /// 2, 4, 8, and optionally 16. 0 indicates that the sample count is not forced.
    pub forced_sample_count: UINT,
}

impl Default for D3D11_RASTERIZER_DESC1 {
    fn default() -> Self {
        D3D11_RASTERIZER_DESC1 {
            fill_mode: D3D11_FILL_MODE::Solid,
            cull_mode: D3D11_CULL_MODE::Back,
            front_counter_clockwise: FALSE,
            depth_bias: 0,
            depth_bias_clamp: 0.,
            slope_scaled_depth_bias: 0.,
            depth_clip_enable: TRUE,
            scissor_enable: FALSE,
            multisample_enable: FALSE,
            antialiased_line_enable: FALSE,
            forced_sample_count: 0,
        }
    }
}
