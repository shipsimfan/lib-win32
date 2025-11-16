use crate::{
    dxgidebug::{
        DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_ID,
        DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    },
    UINT,
};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::DXGI_INFO_QUEUE_FILTER;

/// Describes the types of messages to allow or deny to pass through a filter.
///
/// # Remarks
/// This structure is a member of the [`DXGI_INFO_QUEUE_FILTER`] structure.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    /// The number of message categories to allow or deny.
    pub num_categories: UINT,

    /// An array of [`DXGI_INFO_QUEUE_MESSAGE_CATEGORY`] enumeration values that describe the
    /// message categories to allow or deny. The array must have at least `num_categories` number
    /// of elements.
    pub category_list: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,

    /// The number of message severity levels to allow or deny.
    pub num_severities: UINT,

    /// An array of [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`] enumeration values that describe the
    /// message severity levels to allow or deny. The array must have at least `num_severities`
    /// number of elements.
    pub severity_list: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,

    /// The number of message IDs to allow or deny.
    pub num_ids: UINT,

    /// An array of integers that represent the message IDs to allow or deny. The array must have
    /// at least `num_ids` number of elements.
    pub id_list: *mut DXGI_INFO_QUEUE_MESSAGE_ID,
}

impl Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        DXGI_INFO_QUEUE_FILTER_DESC {
            num_categories: 0,
            category_list: null_mut(),
            num_severities: 0,
            severity_list: null_mut(),
            num_ids: 0,
            id_list: null_mut(),
        }
    }
}
