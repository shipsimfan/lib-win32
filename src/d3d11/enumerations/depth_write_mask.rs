/// Identify the portion of a depth-stencil buffer for writing depth data.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_DEPTH_WRITE_MASK {
    /// Turn off writes to the depth-stencil buffer.
    Zero = 0,

    /// Turn on writes to the depth-stencil buffer.
    All = 1,
}
