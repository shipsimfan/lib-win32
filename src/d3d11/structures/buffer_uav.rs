use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11_BUFFER_UAV_FLAG, D3D11_UNORDERED_ACCESS_VIEW_DESC};

/// Describes the elements in a buffer to use in a unordered-access view.
///
/// # Remarks
/// This structure is used by a [`D3D11_UNORDERED_ACCESS_VIEW_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_UAV {
    /// The zero-based index of the first element to be accessed.
    pub first_element: UINT,

    /// The number of elements in the resource. For structured buffers, this is the number of
    /// structures in the buffer.
    pub num_elements: UINT,

    /// View options for the resource (see [`D3D11_BUFFER_UAV_FLAG`]).
    pub flags: UINT,
}

impl Default for D3D11_BUFFER_UAV {
    fn default() -> Self {
        D3D11_BUFFER_UAV {
            first_element: 0,
            num_elements: 0,
            flags: 0,
        }
    }
}
