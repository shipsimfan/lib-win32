use crate::UINT;
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11DeviceContext, d3dcommon::D3D_FEATURE_LEVEL};

/// Provides access to subresource data.
///
/// # Remarks
/// This structure is used in a call to [`ID3D11DeviceContext::map`].
///
/// The values in these members tell you how much data you can view:
///  - `data` points to row 0 and depth slice 0.
///  - `row_pitch` contains the value that the runtime adds to `data` to move from row to row,
///    where each row contains multiple pixels.
///  - `depth_pitch` contains the value that the runtime adds to `data` to move from depth slice to
///    depth slice, where each depth slice contains multiple rows.
///
/// When `row_pitch` and `depth_pitch` are not appropriate for the resource type, the runtime might
/// set their values to 0. So, don't use these values for anything other than iterating over rows
/// and depth. Here are some examples:
///  - For Buffer and Texture 1D, the runtime assigns values that aren't 0 to `row_pitch` and
///    `depth_pitch`. For example, if a Buffer contains 8 bytes, the runtime assigns values to
///    `row_pitch` and `depth_pitch` that are greater than or equal to 8.
///  - For Texture 2D, the runtime still assigns a value that isn't 0 to `depth_pitch`, assuming
///    that the field isn't used.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    /// Pointer to the data. When [`ID3D11DeviceContext::map`] provides the pointer, the runtime
    /// ensures that the pointer has a specific alignment, depending on the following feature
    /// levels:
    ///  - For [`D3D_FEATURE_LEVEL::_10_0`] and higher, the pointer is aligned to 16 bytes.
    ///  - For lower than [`D3D_FEATURE_LEVEL::_10_0`], the pointer is aligned to 4 bytes.
    pub data: *mut c_void,

    /// The row pitch, or width, or physical size (in bytes) of the data.
    pub row_pitch: UINT,

    /// The depth pitch, or width, or physical size (in bytes)of the data.
    pub depth_pitch: UINT,
}

impl Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        D3D11_MAPPED_SUBRESOURCE {
            data: null_mut(),
            row_pitch: 0,
            depth_pitch: 0,
        }
    }
}
