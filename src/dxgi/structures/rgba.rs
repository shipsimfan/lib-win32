use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_2::{IDXGISwapChain1, DXGI_ALPHA_MODE};

/// Represents a color value with alpha, which is used for transparency.
///
/// # Remarks
/// You can set the members of this structure to values outside the range of 0 through 1 to
/// implement some unusual effects. Values greater than 1 produce strong lights that tend to wash
/// out a scene. Negative values produce dark lights that actually remove light from a scene.
///
/// You can use [`DXGI_RGBA`] with [`IDXGISwapChain1::set_background_color`],
/// [`IDXGISwapChain1::get_background_color`], and [`DXGI_ALPHA_MODE`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct DXGI_RGBA {
    /// Floating-point value that specifies the red component of a color. This value generally is
    /// in the range from 0.0 through 1.0. A value of 0.0 indicates the complete absence of the red
    /// component, while a value of 1.0 indicates that red is fully present.
    pub r: c_float,

    /// Floating-point value that specifies the green component of a color. This value generally is
    /// in the range from 0.0 through 1.0. A value of 0.0 indicates the complete absence of the
    /// green component, while a value of 1.0 indicates that green is fully present.
    pub g: c_float,

    /// Floating-point value that specifies the blue component of a color. This value generally is
    /// in the range from 0.0 through 1.0. A value of 0.0 indicates the complete absence of the
    /// blue component, while a value of 1.0 indicates that blue is fully present.
    pub b: c_float,

    /// Floating-point value that specifies the alpha component of a color. This value generally is
    /// in the range from 0.0 through 1.0. A value of 0.0 indicates fully transparent, while a
    /// value of 1.0 indicates fully opaque.
    pub a: c_float,
}

impl Default for DXGI_RGBA {
    fn default() -> Self {
        DXGI_RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}
