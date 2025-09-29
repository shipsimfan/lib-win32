use crate::{
    dxgi::{DXGI_FORMAT, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL},
    BOOL, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::DXGI_MODE_DESC, FALSE, TRUE};

/// Describes a display mode and whether the display mode supports stereo.
///
/// # Remarks
/// [`DXGI_MODE_DESC1`] is identical to [`DXGI_MODE_DESC`] except that [`DXGI_MODE_DESC1`] includes
/// the `stereo` member.
///
/// This structure is used by the [`IDXGIOutput1::get_display_mode_list1`] and
/// [`IDXGIOutput1::find_closest_matching_mode1`] methods.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_MODE_DESC1 {
    /// A value that describes the resolution width.
    pub width: UINT,

    /// A value that describes the resolution height.
    pub height: UINT,

    /// A [`DXGI_RATIONAL`] structure that describes the refresh rate in hertz.
    pub refresh_rate: DXGI_RATIONAL,

    /// A [`DXGI_FORMAT`]-typed value that describes the display format.
    pub format: DXGI_FORMAT,

    /// A [`DXGI_MODE_SCANLINE_ORDER`]-typed value that describes the scan-line drawing mode.
    pub scanline_ordering: DXGI_MODE_SCANLINE_ORDER,

    /// A [`DXGI_MODE_SCALING`]-typed value that describes the scaling mode.
    pub scaling: DXGI_MODE_SCALING,

    /// Specifies whether the full-screen display mode is stereo. [`TRUE`] if stereo; otherwise,
    /// [`FALSE`].
    pub stereo: BOOL,
}

impl Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        DXGI_MODE_DESC1 {
            width: 0,
            height: 0,
            refresh_rate: DXGI_RATIONAL::default(),
            format: DXGI_FORMAT::Unknown,
            scanline_ordering: DXGI_MODE_SCANLINE_ORDER::Unspecified,
            scaling: DXGI_MODE_SCALING::Unspecified,
            stereo: 0,
        }
    }
}
