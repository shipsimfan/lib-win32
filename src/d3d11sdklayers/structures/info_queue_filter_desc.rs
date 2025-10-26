use crate::{
    d3d11sdklayers::{D3D11_MESSAGE_CATEGORY, D3D11_MESSAGE_ID, D3D11_MESSAGE_SEVERITY},
    UINT,
};
use std::ptr::null_mut;

/// Allow or deny certain types of messages to pass through a filter.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_INFO_QUEUE_FILTER_DESC {
    /// Number of message categories to allow or deny.
    pub num_categories: UINT,

    /// Array of message categories to allow or deny. Array must have at least `num_categories`
    /// members (see [`D3D11_MESSAGE_CATEGORY`]).
    pub category_list: *mut D3D11_MESSAGE_CATEGORY,

    /// Number of message severity levels to allow or deny.
    pub num_severities: UINT,

    /// Array of message severity levels to allow or deny. Array must have at least
    /// `num_severities` members (see [`D3D11_MESSAGE_SEVERITY`]).
    pub severity_list: *mut D3D11_MESSAGE_SEVERITY,

    /// Number of message IDs to allow or deny.
    pub num_ids: UINT,

    /// Array of message IDs to allow or deny. Array must have at least `num_ids` members (see
    /// [`D3D11_MESSAGE_ID`]).
    pub id_list: *mut D3D11_MESSAGE_ID,
}

impl Default for D3D11_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        D3D11_INFO_QUEUE_FILTER_DESC {
            num_categories: 0,
            category_list: null_mut(),
            num_severities: 0,
            severity_list: null_mut(),
            num_ids: 0,
            id_list: null_mut(),
        }
    }
}
