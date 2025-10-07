use crate::UINT;

/// Specifies the subresource from a multisampled 2D texture to use in a render-target view.
///
/// # Remarks
/// Since a multisampled 2D texture contains a single subresource, there is actually nothing to
/// specify in [`D3D11_TEX2DMS_RTV`]. Consequently, `unused_field` is included so that this
/// structure will compile.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_RTV {
    /// Integer of any value.
    pub unused_field: UINT,
}

impl Default for D3D11_TEX2DMS_RTV {
    fn default() -> Self {
        D3D11_TEX2DMS_RTV { unused_field: 0 }
    }
}
