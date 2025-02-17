use crate::{
    dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGISurface;

/// Describes a surface.
///
/// This structure is used by the [`IDXGISurface::get_desc`] and [`IDXGIDevice::create_surface`]
/// methods.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SURFACE_DESC {
    /// A value describing the surface width.
    pub width: UINT,

    /// A value describing the surface height.
    pub height: UINT,

    /// A member of the [`DXGI_FORMAT`] enumerated type that describes the surface format.
    pub format: DXGI_FORMAT,

    /// A member of the [`DXGI_SAMPLE_DESC`] structure that describes multi-sampling parameters for
    /// the surface.
    pub sample_desc: DXGI_SAMPLE_DESC,
}

impl Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        DXGI_SURFACE_DESC {
            width: 0,
            height: 0,
            format: DXGI_FORMAT::Unknown,
            sample_desc: DXGI_SAMPLE_DESC::default(),
        }
    }
}
