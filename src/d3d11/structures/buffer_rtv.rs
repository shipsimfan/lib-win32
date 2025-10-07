use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_RENDER_TARGET_VIEW_DESC};

/// Specifies the elements in a buffer resource to use in a render-target view.
///
/// # Remarks
/// A render-target view is a member of a render-target-view description (see
/// [`D3D11_RENDER_TARGET_VIEW_DESC`]). Create a render-target view by calling
/// [`ID3D11Device::create_render_target_view`].
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_RTV {
    #[allow(missing_docs)]
    pub u1: D3D11_BUFFER_RTV_UNION1,

    #[allow(missing_docs)]
    pub u2: D3D11_BUFFER_RTV_UNION2,
}

impl Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        D3D11_BUFFER_RTV {
            u1: D3D11_BUFFER_RTV_UNION1::default(),
            u2: D3D11_BUFFER_RTV_UNION2::default(),
        }
    }
}

/// The first element of [`D3D11_BUFFER_RTV`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_RTV_UNION1 {
    /// Number of bytes between the beginning of the buffer and the first element to access.
    pub first_element: UINT,

    /// The offset of the first element in the view to access, relative to element 0.
    pub element_offset: UINT,
}

impl Default for D3D11_BUFFER_RTV_UNION1 {
    fn default() -> Self {
        D3D11_BUFFER_RTV_UNION1 { first_element: 0 }
    }
}

/// The second element of [`D3D11_BUFFER_RTV`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_RTV_UNION2 {
    /// The total number of elements in the view.
    pub num_elements: UINT,

    /// The width of each element (in bytes). This can be determined from the format stored in the
    /// render-target-view description.
    pub element_width: UINT,
}

impl Default for D3D11_BUFFER_RTV_UNION2 {
    fn default() -> Self {
        D3D11_BUFFER_RTV_UNION2 { num_elements: 0 }
    }
}
