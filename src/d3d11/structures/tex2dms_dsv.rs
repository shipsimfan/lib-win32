use crate::UINT;

/// Specifies the subresource from a multisampled 2D texture that is accessible to a depth-stencil
/// view.
///
/// # Remarks
/// Because a multisampled 2D texture contains a single subtexture, there is nothing to specify;
/// this unused member is included so that this structure will compile in C.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_DSV {
    /// Unused.
    pub unused_field: UINT,
}

impl Default for D3D11_TEX2DMS_DSV {
    fn default() -> Self {
        D3D11_TEX2DMS_DSV { unused_field: 0 }
    }
}
