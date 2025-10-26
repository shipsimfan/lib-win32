// rustdoc imports
#[allow(unused_imports)]
use crate::d3dcommon::ID3DInclude;

/// Values that indicate the location of a shader `#include` file.
///
/// # Remarks
/// You pass a [`D3D_INCLUDE_TYPE`]-typed value to the `include_type` parameter in a call to the
/// [`ID3DInclude::open`] method to indicate the location of the `#include` file.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_INCLUDE_TYPE {
    /// The local directory.
    Local = 0,

    /// The system directory.
    System,

    /// The local directory.
    D3D10Local,

    /// The system directory.
    D3D10System,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits.
    ///
    /// Do not use this value.
    ForceDword = 0x7FFFFFFF,
}
