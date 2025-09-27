// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Resource;

/// Identifies the type of resource being used.
///
/// This enumeration is used in [`ID3D11Resource::get_type`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_RESOURCE_DIMENSION {
    /// Resource is of unknown type.
    Unknown = 0,

    /// Resource is a buffer.
    Buffer = 1,

    /// Resource is a 1D texture.
    Texture1D = 2,

    /// Resource is a 2D texture.
    Texture2D = 3,

    /// Resource is a 3D texture.
    Texture3D = 4,
}
