use crate::UINT;

/// Specifies the subresources from a multisampled 2D texture to use in a shader-resource view.
///
/// # Remarks
/// Since a multisampled 2D texture contains a single subresource, there is actually nothing to
/// specify in [`D3D11_TEX2DMS_SRV`]. Consequently, `unused_field` is included so that this
/// structure will compile in C.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_SRV {
    /// Integer of any value.
    pub unused_field: UINT,
}

impl Default for D3D11_TEX2DMS_SRV {
    fn default() -> Self {
        D3D11_TEX2DMS_SRV { unused_field: 0 }
    }
}
