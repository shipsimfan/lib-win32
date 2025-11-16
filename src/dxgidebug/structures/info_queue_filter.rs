use crate::dxgidebug::DXGI_INFO_QUEUE_FILTER_DESC;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::IDXGIInfoQueue;

/// Describes a debug message filter, which contains lists of message types to allow and deny.
///
/// # Remarks
/// Use with an [`IDXGIInfoQueue`] interface.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_INFO_QUEUE_FILTER {
    /// A [`DXGI_INFO_QUEUE_FILTER_DESC`] structure that describes the types of messages to allow.
    pub allow_list: DXGI_INFO_QUEUE_FILTER_DESC,

    /// A [`DXGI_INFO_QUEUE_FILTER_DESC`] structure that describes the types of messages to deny.
    pub deny_list: DXGI_INFO_QUEUE_FILTER_DESC,
}

impl Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        DXGI_INFO_QUEUE_FILTER {
            allow_list: DXGI_INFO_QUEUE_FILTER_DESC::default(),
            deny_list: DXGI_INFO_QUEUE_FILTER_DESC::default(),
        }
    }
}
