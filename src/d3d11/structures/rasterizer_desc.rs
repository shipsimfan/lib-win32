use crate::{
    d3d11::{D3D11_CULL_MODE, D3D11_FILL_MODE},
    BOOL, FALSE, FLOAT, INT, TRUE,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

/// Describes rasterizer state.
///
/// # Remarks
/// Rasterizer state defines the behavior of the rasterizer stage. To create a rasterizer-state
/// object, call [`ID3D11Device::create_rasterizer_state`]. To set rasterizer state, call
/// [`ID3D11DeviceContext::rs_set_state`].
///
/// The settings of the `multisample_enable` and `antialiased_line_enable` members apply only to
/// multisample antialiasing (MSAA) render targets (that is, render targets with sample counts
/// greater than 1). Because of the differences in feature-level behavior and as long as you aren’t
/// performing any line drawing or don’t mind that lines render as quadrilaterals, we recommend
/// that you always set `multisample_enable` to [`TRUE`] whenever you render on MSAA render
/// targets.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_RASTERIZER_DESC {
    /// Determines the fill mode to use when rendering (see [`D3D11_FILL_MODE`]).
    pub fill_mode: D3D11_FILL_MODE,

    /// Indicates triangles facing the specified direction are not drawn (see [`D3D11_CULL_MODE`]).
    pub cull_mode: D3D11_CULL_MODE,

    /// Determines if a triangle is front- or back-facing. If this parameter is [`TRUE`], a
    /// triangle will be considered front-facing if its vertices are counter-clockwise on the
    /// render target and considered back-facing if they are clockwise. If this parameter is
    /// [`FALSE`], the opposite is true.
    pub front_counter_clockwise: BOOL,

    /// Depth value added to a given pixel.
    pub depth_bias: INT,

    /// Maximum depth bias of a pixel.
    pub depth_bias_clamp: FLOAT,

    /// Scalar on a given pixel's slope.
    pub slope_scaled_depth_bias: FLOAT,

    /// Enable clipping based on distance.
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

    /// Enable scissor-rectangle culling. All pixels outside an active scissor rectangle are
    /// culled.
    pub scissor_enable: BOOL,

    /// Specifies whether to use the quadrilateral or alpha line anti-aliasing algorithm on
    /// multisample antialiasing (MSAA) render targets. Set to [`TRUE`] to use the quadrilateral
    /// line anti-aliasing algorithm and to [`FALSE`] to use the alpha line anti-aliasing
    /// algorithm.
    pub multisample_enable: BOOL,

    /// Specifies whether to enable line antialiasing; only applies if doing line drawing and
    /// `multisample_enable` is [`FALSE`].
    pub antialiased_line_enable: BOOL,
}

impl Default for D3D11_RASTERIZER_DESC {
    fn default() -> Self {
        D3D11_RASTERIZER_DESC {
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
        }
    }
}
