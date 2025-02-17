use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_GAMMA_CONTROL;

/// Represents an RGB color.
///
/// This structure is a member of the [`DXGI_GAMMA_CONTROL`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct DXGI_RGB {
    /// A value representing the color of the red component. The range of this value is between 0
    /// and 1.
    pub red: c_float,

    /// A value representing the color of the green component. The range of this value is between 0
    /// and 1.
    pub green: c_float,

    /// A value representing the color of the blue component. The range of this value is between 0
    /// and 1.
    pub blue: c_float,
}

impl Default for DXGI_RGB {
    fn default() -> Self {
        DXGI_RGB {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}
