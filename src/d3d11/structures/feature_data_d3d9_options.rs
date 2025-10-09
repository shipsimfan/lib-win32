use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// Describes Direct3D 9 feature options in the current graphics driver.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
    /// Specifies whether the driver supports the nonpowers-of-2-unconditionally feature. The
    /// runtime sets this member to [`TRUE`] for hardware at Direct3D 10 and higher feature levels.
    /// For hardware at Direct3D 9.3 and lower feature levels, the runtime sets this member to
    /// [`FALSE`] if the hardware and driver support the powers-of-2 (2D textures must have widths
    /// and heights specified as powers of two) feature or the nonpowers-of-2-conditionally
    /// feature.
    pub full_non_pow_2_texture_support: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D9_OPTIONS {
            full_non_pow_2_texture_support: 0,
        }
    }
}
