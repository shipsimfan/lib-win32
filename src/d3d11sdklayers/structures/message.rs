use crate::{
    d3d11sdklayers::{D3D11_MESSAGE_CATEGORY, D3D11_MESSAGE_ID, D3D11_MESSAGE_SEVERITY},
    SIZE_T,
};
use std::{ffi::c_char, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11sdklayers::ID3D11InfoQueue;

/// A debug message in the Information Queue.
///
/// # Remarks
/// This structure is returned from [`ID3D11InfoQueue::get_message`] as part of the Information
/// Queue feature (see [`ID3D11InfoQueue`] Interface).
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_MESSAGE {
    /// The category of the message. See [`D3D11_MESSAGE_CATEGORY`].
    pub category: D3D11_MESSAGE_CATEGORY,

    /// The severity of the message. See [`D3D11_MESSAGE_SEVERITY`].
    pub severity: D3D11_MESSAGE_SEVERITY,

    /// The ID of the message. See [`D3D11_MESSAGE_ID`].
    pub id: D3D11_MESSAGE_ID,

    /// The message string.
    pub description: *const c_char,

    /// The length of `description` in bytes.
    pub description_byte_length: SIZE_T,
}

impl Default for D3D11_MESSAGE {
    fn default() -> Self {
        D3D11_MESSAGE {
            category: D3D11_MESSAGE_CATEGORY::ApplicationDefined,
            severity: D3D11_MESSAGE_SEVERITY::Corruption,
            id: D3D11_MESSAGE_ID::Unknown,
            description: null(),
            description_byte_length: 0,
        }
    }
}
