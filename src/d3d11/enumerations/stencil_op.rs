/// The stencil operations that can be performed during depth-stencil testing.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_STENCIL_OP {
    /// Keep the existing stencil data.
    Keep = 1,

    /// Set the stencil data to 0.
    Zero = 2,

    /// Set the stencil data to the reference value set by calling
    /// [`ID3D11DeviceContext::om_set_depth_stencil_state`].
    Replace = 3,

    /// Increment the stencil value by 1, and clamp the result.
    IncrSat = 4,

    /// Decrement the stencil value by 1, and clamp the result.
    DecrSat = 5,

    /// Invert the stencil data.
    Invert = 6,

    /// Increment the stencil value by 1, and wrap the result if necessary.
    Incr = 7,

    /// Decrement the stencil value by 1, and wrap the result if necessary.
    Decr = 8,
}
