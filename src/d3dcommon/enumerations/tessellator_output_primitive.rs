// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SHADER_DESC;

/// Output primitive types.
///
/// # Remarks
/// The output primitive type determines how the tessellator output data is organized; this
/// enumeration is used by [`D3D11_SHADER_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    #[allow(missing_docs)]
    Undefined = 0,

    #[allow(missing_docs)]
    Point = 1,

    #[allow(missing_docs)]
    Line = 2,

    #[allow(missing_docs)]
    TriangleCw = 3,

    #[allow(missing_docs)]
    TriangleCcw = 4,

    /// The output primitive type is undefined.
    D3D11Undefined,

    /// The output primitive type is a point.
    D3D11Point,

    /// The output primitive type is a line.
    D3D11Line,

    /// The output primitive type is a clockwise triangle.
    D3D11TriangleCw,

    /// The output primitive type is a counter clockwise triangle.
    D3D11TriangleCcw,
}
