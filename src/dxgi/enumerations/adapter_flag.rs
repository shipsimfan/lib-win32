// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::DXGI_ADAPTER_DESC1, dxgi1_2::DXGI_ADAPTER_DESC2};

/// Identifies the type of DXGI adapter.
///
/// # Remarks
/// The [`DXGI_ADAPTER_FLAG`] enumerated type is used by the Flags member of the
/// [`DXGI_ADAPTER_DESC1`] or [`DXGI_ADAPTER_DESC2`] structure to identify the type of DXGI
/// adapter.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_ADAPTER_FLAG {
    /// Specifies no flags.
    None = 0,

    /// Value always set to 0. This flag is reserved.
    Remote = 1,

    /// Specifies a software adapter.
    Software = 2,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. This value is not
    /// used.
    ForceDWord = 0xFFFFFFFF,
}
