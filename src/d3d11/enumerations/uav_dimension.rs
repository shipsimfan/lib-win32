// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_UNORDERED_ACCESS_VIEW_DESC;

/// Unordered-access view options.
///
/// # Remarks
/// This enumeration is used by a unordered access-view description (see
/// [`D3D11_UNORDERED_ACCESS_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_UAV_DIMENSION {
    /// The view type is unknown.
    Unknown = 0,

    /// View the resource as a buffer.
    Buffer = 1,

    /// View the resource as a 1D texture.
    Texture1D = 2,

    /// View the resource as a 1D texture array.
    Texture1DArray = 3,

    /// View the resource as a 2D texture.
    Texture2D = 4,

    /// View the resource as a 2D texture array.
    Texture2DArray = 5,

    /// View the resource as a 3D texture array.
    Texture3D = 8,
}
