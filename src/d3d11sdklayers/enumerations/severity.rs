// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11sdklayers::{ID3D11InfoQueue, D3D11_INFO_QUEUE_FILTER};

/// Debug message severity levels for an information queue.
///
/// # Remarks
/// Use these values to allow or deny message categories to pass through the storage and retrieval
/// filters for an information queue (see [`D3D11_INFO_QUEUE_FILTER`]). This API is used by
/// [`ID3D11InfoQueue::add_application_message`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_MESSAGE_SEVERITY {
    /// Defines some type of corruption which has occurred.
    Corruption = 0,

    /// Defines an error message.
    Error,

    /// Defines a warning message.
    Warning,

    /// Defines an information message.
    Info,

    /// Defines a message other than corruption, error, warning, or information.
    Message,
}
