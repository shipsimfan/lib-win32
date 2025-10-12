// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SHADER_DESC;

/// Domain options for tessellator data.
///
/// # Remarks
/// The data domain defines the type of data. This enumeration is used by [`D3D11_SHADER_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_TESSELLATOR_DOMAIN {
    #[allow(missing_docs)]
    Undefined = 0,

    #[allow(missing_docs)]
    Isoline = 1,

    #[allow(missing_docs)]
    Tri = 2,

    #[allow(missing_docs)]
    Quad = 3,

    /// The data type is undefined.
    D3D11Undefined,

    /// Isoline data.
    D3D11Isoline,

    /// Triangle data.
    D3D11Tri,

    /// Quad data.
    D3D11Quad,
}
