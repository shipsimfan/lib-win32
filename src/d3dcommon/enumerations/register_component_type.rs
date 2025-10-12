// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SIGNATURE_PARAMETER_DESC;

/// Values that identify the data types that can be stored in a register.
///
/// # Remarks
/// A register component type is specified in the `component_type` member of the
/// [`D3D11_SIGNATURE_PARAMETER_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_REGISTER_COMPONENT_TYPE {
    /// The data type is unknown.
    Unknown = 0,

    /// 32-bit unsigned integer.
    UInt32 = 1,

    /// 32-bit signed integer.
    SInt32 = 2,

    /// 32-bit floating-point number.
    Float32 = 3,

    #[allow(missing_docs)]
    UInt16,

    #[allow(missing_docs)]
    SInt16,

    #[allow(missing_docs)]
    Float16,

    #[allow(missing_docs)]
    UInt64,

    #[allow(missing_docs)]
    SInt64,

    #[allow(missing_docs)]
    Float64,

    /// The data type is unknown.
    D3D10Unknown,

    /// 32-bit unsigned integer.
    D3D10UInt32,

    /// 32-bit signed integer.
    D3D10SInt32,

    /// 32-bit floating-point number.
    D3D10Float32,

    #[allow(missing_docs)]
    D3D10UInt16,

    #[allow(missing_docs)]
    D3D10SInt16,

    #[allow(missing_docs)]
    D3D10Float16,

    #[allow(missing_docs)]
    D3D10UInt64,

    #[allow(missing_docs)]
    D3D10SInt64,

    #[allow(missing_docs)]
    D3D10Float64,
}
