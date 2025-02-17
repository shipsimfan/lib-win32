use crate::{BYTE, INT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGISurface;

/// Describes a mapped rectangle that is used to access a surface.
///
/// The [`DXGI_MAPPED_RECT`] structure is initialized by the [`IDXGISurface::map`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_MAPPED_RECT {
    /// A value that describes the width, in bytes, of the surface.
    pub pitch: INT,

    /// A pointer to the image buffer of the surface.
    pub bits: *mut BYTE,
}

impl Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        DXGI_MAPPED_RECT {
            pitch: 0,
            bits: null_mut(),
        }
    }
}
