use crate::d3d11sdklayers::D3D11_INFO_QUEUE_FILTER_DESC;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11sdklayers::ID3D11InfoQueue;

/// Debug message filter; contains a lists of message types to allow or deny.
///
/// # Remarks
/// For use with an [`ID3D11InfoQueue`] Interface.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_INFO_QUEUE_FILTER {
    /// Types of messages that you want to allow. See [`D3D11_INFO_QUEUE_FILTER_DESC`].
    pub allow_list: D3D11_INFO_QUEUE_FILTER_DESC,

    /// Types of messages that you want to deny.
    pub deny_list: D3D11_INFO_QUEUE_FILTER_DESC,
}

impl Default for D3D11_INFO_QUEUE_FILTER {
    fn default() -> Self {
        D3D11_INFO_QUEUE_FILTER {
            allow_list: D3D11_INFO_QUEUE_FILTER_DESC::default(),
            deny_list: D3D11_INFO_QUEUE_FILTER_DESC::default(),
        }
    }
}
