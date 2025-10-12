// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SHADER_TYPE_DESC;

/// Values that identify the class of a shader variable.
///
/// # Remarks
/// The class of a shader variable is not a programming class; the class identifies the variable
/// class such as scalar, vector, object, and so on. [`D3D_SHADER_VARIABLE_CLASS`]-typed values are
/// specified in the Class member of the [`D3D11_SHADER_TYPE_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_SHADER_VARIABLE_CLASS {
    /// The shader variable is a scalar.
    Scalar = 0,

    /// The shader variable is a vector.
    Vector,

    /// The shader variable is a row-major matrix.
    MatrixRows,

    /// The shader variable is a column-major matrix.
    MatrixColumns,

    /// The shader variable is an object.
    Object,

    /// The shader variable is a structure.
    Struct,

    /// The shader variable is a class.
    InterfaceClass,

    /// The shader variable is an interface.
    InterfacePointer,

    /// The shader variable is a scalar.
    D3D10Scalar,

    /// The shader variable is a vector.
    D3D10Vector,

    /// The shader variable is a row-major matrix.
    D3D10MatrixRows,

    /// The shader variable is a column-major matrix.
    D3D10MatrixColumns,

    /// The shader variable is an object.
    D3D10Object,

    /// The shader variable is a structure.
    D3D10Struct,

    /// The shader variable is a class.
    D3D11InterfaceClass,

    /// The shader variable is an interface.
    D3D11InterfacePointer,

    /// This value is not used by a programmer; it exists to force the enumeration to compile to 32
    /// bits.
    ForceDword = 0x7FFFFFFF,
}
