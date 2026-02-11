use crate::UINT32;

// rustdoc imports
#[allow(unused_imports)]
use crate::DISPLAYCONFIG_VIDEO_SIGNAL_INFO;

/// The [`DISPLAYCONFIG_RATIONAL`] structure describes a fractional value that represents vertical
/// and horizontal frequencies of a video mode (that is, vertical sync and horizontal sync).
///
/// # Remarks
/// A [`DISPLAYCONFIG_RATIONAL`] structure is specified in members of the
/// [`DISPLAYCONFIG_PATH_TARGET_INFO`] and [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structures.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DISPLAYCONFIG_RATIONAL {
    /// The numerator of the frequency fraction.
    pub numerator: UINT32,

    /// The denominator of the frequency fraction.
    pub denominator: UINT32,
}

impl Default for DISPLAYCONFIG_RATIONAL {
    fn default() -> Self {
        DISPLAYCONFIG_RATIONAL {
            numerator: 0,
            denominator: 1,
        }
    }
}
