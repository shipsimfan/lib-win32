use crate::{d3d11::D3D11_RENDER_TARGET_BLEND_DESC, BOOL, FALSE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::ID3D11Device,
    d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL},
    dxgi::DXGI_FORMAT,
    TRUE,
};

/// Describes the blend state that you use in a call to [`ID3D11Device::create_blend_state`] to
/// create a blend-state object.
///
/// # Remarks
///  If the driver type is set to [`D3D_DRIVER_TYPE::Hardware`], the feature level is set to less
/// than or equal to [`D3D_FEATURE_LEVEL::_9_3`], and the pixel format of the render target is set
/// to [`DXGI_FORMAT::R8G8B8A8UNormSRGB`], [`DXGI_FORMAT::B8G8R8A8UNormSRGB`], or
/// [`DXGI_FORMAT::B8G8R8X8UNormSRGB`], the display device performs the blend in standard RGB
/// (sRGB) space and not in linear space. However, if the feature level is set to greater than
/// [`D3D_FEATURE_LEVEL::_9_3`], the display device performs the blend in linear space, which is
/// ideal.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_BLEND_DESC {
    /// Specifies whether to use alpha-to-coverage as a multisampling technique when setting a
    /// pixel to a render target.
    pub alpha_to_converge_enable: BOOL,

    /// Specifies whether to enable independent blending in simultaneous render targets. Set to
    /// [`TRUE`] to enable independent blending. If set to [`FALSE`], only the `render_target[0]`
    /// members are used; `render_target[1..7]` are ignored.
    pub independent_blend_enable: BOOL,

    /// An array of [`D3D11_RENDER_TARGET_BLEND_DESC`] structures that describe the blend states
    /// for render targets; these correspond to the eight render targets that can be bound to the
    /// output-merger stage at one time.
    pub render_target: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}

impl Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        D3D11_BLEND_DESC {
            alpha_to_converge_enable: FALSE,
            independent_blend_enable: FALSE,
            render_target: [
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
                D3D11_RENDER_TARGET_BLEND_DESC::default(),
            ],
        }
    }
}
