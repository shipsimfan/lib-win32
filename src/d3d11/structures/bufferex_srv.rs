use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11_BUFFEREX_SRV_FLAG, D3D11_SHADER_RESOURCE_VIEW_DESC};

/// Describes the elements in a raw buffer resource to use in a shader-resource view.
///
/// # Remarks
/// This structure is used by [`D3D11_SHADER_RESOURCE_VIEW_DESC`] to create a raw view of a buffer.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFEREX_SRV {
    /// The index of the first element to be accessed by the view.
    pub first_element: UINT,

    /// The number of elements in the resource.
    pub num_elements: UINT,

    /// A [`D3D11_BUFFEREX_SRV_FLAG`]-typed value that identifies view options for the buffer.
    /// Currently, the only option is to identify a raw view of the buffer.
    pub flags: UINT,
}

impl Default for D3D11_BUFFEREX_SRV {
    fn default() -> Self {
        D3D11_BUFFEREX_SRV {
            first_element: 0,
            num_elements: 0,
            flags: 0,
        }
    }
}
