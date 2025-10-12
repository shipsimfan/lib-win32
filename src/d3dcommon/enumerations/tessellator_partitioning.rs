// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SHADER_DESC;

/// Partitioning options.
///
/// # Remarks
/// During tessellation, the partition option helps to determine how the algorithm chooses the next
/// partition value; this enumeration is used by [`D3D11_SHADER_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_TESSELLATOR_PARTITIONING {
    #[allow(missing_docs)]
    Undefined = 0,

    #[allow(missing_docs)]
    Integer = 1,

    #[allow(missing_docs)]
    Pow2 = 2,

    #[allow(missing_docs)]
    FractionalOdd = 3,

    #[allow(missing_docs)]
    FractionalEven = 4,

    /// The partitioning type is undefined.
    D3D11Undefined,

    /// Partition with integers only.
    D3D11Integer,

    /// Partition with a power-of-two number only.
    D3D11Pow2,

    /// Partition with an odd, fractional number.
    D3D11FractionalOdd,

    /// Partition with an even, fractional number.
    D3D11FractionalEven,
}
