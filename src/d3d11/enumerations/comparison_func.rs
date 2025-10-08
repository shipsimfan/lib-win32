// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_SAMPLER_DESC;

/// Comparison options.
///
/// # Remarks
/// A comparison option determines whether how the runtime compares source (new) data against
/// destination (existing) data before storing the new data. The comparison option is declared in a
/// description before an object is created. The API allows you to set a comparison option for a
/// depth-stencil buffer (see [`D3D11_DEPTH_STENCIL_DESC`]), depth-stencil operations (see
/// [`D3D11_DEPTH_STENCILOP_DESC`]), or sampler state (see [`D3D11_SAMPLER_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_COMPARISON_FUNC {
    /// Never pass the comparison.
    Never = 1,

    /// If the source data is less than the destination data, the comparison passes.
    Less = 2,

    /// If the source data is equal to the destination data, the comparison passes.
    Equal = 3,

    /// If the source data is less than or equal to the destination data, the comparison passes.
    LessEqual = 4,

    /// If the source data is greater than the destination data, the comparison passes.
    Greater = 5,

    /// If the source data is not equal to the destination data, the comparison passes.
    NotEqual = 6,

    /// If the source data is greater than or equal to the destination data, the comparison passes.
    GreaterEqual = 7,

    /// Always pass the comparison.
    Always = 8,
}
