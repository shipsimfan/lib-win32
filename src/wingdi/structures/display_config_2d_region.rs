use crate::UINT32;

/// The [`DISPLAYCONFIG_2DREGION`] structure represents a point or an offset in a two-dimensional
/// space.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DISPLAYCONFIG_2DREGION {
    /// The horizontal component of the point or offset.
    pub cx: UINT32,

    /// The vertical component of the point or offset.
    pub cy: UINT32,
}

impl Default for DISPLAYCONFIG_2DREGION {
    fn default() -> Self {
        DISPLAYCONFIG_2DREGION { cx: 0, cy: 0 }
    }
}
