use crate::{
    dxgidebug::{
        DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_ID,
        DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    },
    SIZE_T,
};
use std::{ffi::c_char, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::IDXGIInfoQueue;

/// Describes a debug message in the information queue.
///
/// # Remarks
/// [`IDXGIInfoQueue::get_message`] returns a pointer to this structure.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    /// A [`DXGI_DEBUG_ID`] value that identifies the entity that produced the message.
    pub producer: DXGI_DEBUG_ID,

    /// A [`DXGI_INFO_QUEUE_MESSAGE_CATEGORY`]-typed value that specifies the category of the
    /// message.
    pub category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,

    /// A [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`]-typed value that specifies the severity of the
    /// message.
    pub severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,

    /// An integer that uniquely identifies the message.
    pub id: DXGI_INFO_QUEUE_MESSAGE_ID,

    /// The message string.
    pub description: *const c_char,

    /// The length of the message string at `description`, in bytes.
    pub description_byte_length: SIZE_T,
}

impl Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        DXGI_INFO_QUEUE_MESSAGE {
            producer: DXGI_DEBUG_ID::default(),
            category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY::Unknown,
            severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Corruption,
            id: 0,
            description: null(),
            description_byte_length: 0,
        }
    }
}
