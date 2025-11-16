// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::IDXGIInfoQueue;

/// Values that specify debug message severity levels for an information queue.
///
/// # Remarks
/// Use this enumeration when you call [`IDXGIInfoQueue::get_message`] to retrieve a message and
/// when you call [`IDXGIInfoQueue::add_message`] to add a message. Also, use this enumeration with
/// [`IDXGIInfoQueue::add_application_message`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    /// Defines some type of corruption that has occurred.
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
