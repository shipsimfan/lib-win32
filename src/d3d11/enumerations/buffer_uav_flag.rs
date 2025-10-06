// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_FORMAT;

/// Identifies unordered-access view options for a buffer resource.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_BUFFER_UAV_FLAG {
    /// Resource contains raw, unstructured data. Requires the UAV format to be
    /// [`DXGI_FORMAT::R32Typeless`].
    Raw = 0x1,

    /// Allow data to be appended to the end of the buffer. [`D3D11_BUFFER_UAV_FLAG::Append`] flag
    /// must also be used for any view that will be used as a `AppendStructuredBuffer` or a
    /// `ConsumeStructuredBuffer`. Requires the UAV format to be [`DXGI_FORMAT::Unknown`].
    Append = 0x2,

    /// Adds a counter to the unordered-access-view buffer. [`D3D11_BUFFER_UAV_FLAG::Counter`] can
    /// only be used on a UAV that is a `RWStructuredBuffer` and it enables the functionality
    /// needed for the `IncrementCounter` and `DecrementCounter` methods in HLSL. Requires the UAV
    /// format to be [`DXGI_FORMAT::Unknown`].
    Counter = 0x4,
}
