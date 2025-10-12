/// Values that identify the intended use of constant-buffer data.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_CBUFFER_TYPE {
    /// A buffer containing scalar constants.
    CBuffer = 0,

    /// A buffer containing texture data.
    TBuffer,

    /// A buffer containing interface pointers.
    InterfacePointers,

    /// A buffer containing binding information.
    ResourceBindInfo,

    /// A buffer containing scalar constants.
    D3D10CBuffer,

    /// A buffer containing texture data.
    D3D10TBuffer,

    /// A buffer containing scalar constants.
    D3D11CBuffer,

    /// A buffer containing texture data.
    D3D11TBuffer,

    /// A buffer containing interface pointers.
    D3D11InterfacePointers,

    /// A buffer containing binding information.
    D3D11ResourceBindInfo,
}
