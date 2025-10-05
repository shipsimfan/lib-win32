use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_SHADER_RESOURCE_VIEW_DESC};

/// Specifies the elements in a buffer resource to use in a shader-resource view.
///
/// # Remarks
/// The [`D3D11_BUFFER_SRV`] structure is a member of the [`D3D11_SHADER_RESOURCE_VIEW_DESC`]
/// structure, which represents a shader-resource view description. You can create a
/// shader-resource view by calling the [`ID3D11Device::create_shader_resource_view`] method.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_SRV {
    #[allow(missing_docs)]
    pub u1: D3D11_BUFFER_SRV_UNION1,

    #[allow(missing_docs)]
    pub u2: D3D11_BUFFER_SRV_UNION2,
}

impl Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        D3D11_BUFFER_SRV {
            u1: D3D11_BUFFER_SRV_UNION1::default(),
            u2: D3D11_BUFFER_SRV_UNION2::default(),
        }
    }
}

/// The first element of [`D3D11_BUFFER_SRV`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_SRV_UNION1 {
    /// Index of the first element to access.
    pub first_element: UINT,

    /// The offset of the first element in the view to access, relative to element 0.
    pub element_offset: UINT,
}

impl Default for D3D11_BUFFER_SRV_UNION1 {
    fn default() -> Self {
        D3D11_BUFFER_SRV_UNION1 { first_element: 0 }
    }
}

/// The second element of [`D3D11_BUFFER_SRV`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_SRV_UNION2 {
    /// The total number of elements in the view.
    pub num_elements: UINT,

    /// The width of each element (in bytes). This can be determined from the format stored in the
    /// shader-resource-view description.
    pub element_width: UINT,
}

impl Default for D3D11_BUFFER_SRV_UNION2 {
    fn default() -> Self {
        D3D11_BUFFER_SRV_UNION2 { num_elements: 0 }
    }
}
