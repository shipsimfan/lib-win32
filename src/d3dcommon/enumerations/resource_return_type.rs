/// Indicates return value type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_RESOURCE_RETURN_TYPE {
    #[allow(missing_docs)]
    UNorm = 1,

    #[allow(missing_docs)]
    SNorm = 2,

    #[allow(missing_docs)]
    SInt = 3,

    #[allow(missing_docs)]
    UInt = 4,

    #[allow(missing_docs)]
    Float = 5,

    #[allow(missing_docs)]
    Mixed = 6,

    #[allow(missing_docs)]
    Double = 7,

    #[allow(missing_docs)]
    Continued = 8,

    #[allow(missing_docs)]
    D3D10UNorm,

    #[allow(missing_docs)]
    D3D10SNorm,

    #[allow(missing_docs)]
    D3D10SInt,

    #[allow(missing_docs)]
    D3D10UInt,

    #[allow(missing_docs)]
    D3D10Float,

    #[allow(missing_docs)]
    D3D10Mixed,

    /// Return type is `UNORM`.
    D3D11UNorm,

    /// Return type is `SNORM`.
    D3D11SNorm,

    /// Return type is `SINT`.
    D3D11SInt,

    /// Return type is `UINT`.
    D3D11UInt,

    /// Return type is `FLOAT`.
    D3D11Float,

    /// Return type is unknown.
    D3D11Mixed,

    /// Return type is `DOUBLE`.
    D3D11Double,

    /// Return type is a multiple-dword type, such as a double or uint64, and the component is
    /// continued from the previous component that was declared. The first component represents the
    /// lower bits.
    D3D11Continued,
}
