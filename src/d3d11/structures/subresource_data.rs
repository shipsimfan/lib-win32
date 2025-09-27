use crate::UINT;
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Specifies data for initializing a subresource.
///
/// # Remarks
/// This structure is used in calls to create buffers ([`ID3D11Device::create_buffer`]) and
/// textures ([`ID3D11Device::create_texture_1d`], [`ID3D11Device::create_texture_2d`], and
/// [`ID3D11Device::create_texture_3d`]). If the resource you create does not require a
/// system-memory pitch or a system-memory-slice pitch, you can use those members to pass size
/// information, which might help you when you debug a problem with creating a resource.
///
/// A subresource is a single mipmap-level surface. You can pass an array of subresources to one of
/// the preceding methods to create the resource. A subresource can be 1D, 2D, or 3D. How you set
/// the members of [`D3D11_SUBRESOURCE_DATA`] depend on whether the subresource is 1D, 2D, or 3D.
///
/// The x, y, and d values are 0-based indices and `byte_per_pixel` depends on the pixel format.
/// For mipmapped 3D surfaces, the number of depth slices in each level is half the number of the
/// previous level (minimum 1) and rounded down if dividing by two results in a non-whole number.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SUBRESOURCE_DATA {
    /// Pointer to the initialization data.
    pub sys_mem: *const c_void,

    /// The distance (in bytes) from the beginning of one line of a texture to the next line.
    /// System-memory pitch is used only for 2D and 3D texture data as it is has no meaning for the
    ///  other resource types. Specify the distance from the first pixel of one 2D slice of a 3D
    /// texture to the first pixel of the next 2D slice in that texture in the
    /// `sys_mem_slice_pitch` member.
    pub sys_mem_pitch: UINT,

    /// The distance (in bytes) from the beginning of one depth level to the next.
    /// System-memory-slice pitch is only used for 3D texture data as it has no meaning for the
    /// other resource types.
    pub sys_mem_slice_pitch: UINT,
}

impl Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        D3D11_SUBRESOURCE_DATA {
            sys_mem: null(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        }
    }
}
