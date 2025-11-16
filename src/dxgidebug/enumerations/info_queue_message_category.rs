// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::IDXGIInfoQueue;

/// Values that specify categories of debug messages.
///
/// # Remarks
/// Use this enumeration when you call [`IDXGIInfoQueue::get_message`] to retrieve a message and
/// when you call [`IDXGIInfoQueue::add_message`] to add a message. When you create an info queue
/// filter, you can use these values to allow or deny any categories of messages to pass through
/// the storage and retrieval filters.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    /// Unknown category.
    Unknown = 0,

    /// Miscellaneous category.
    Miscellaneous,

    /// Initialization category.
    Initialization,

    /// Cleanup category.
    Cleanup,

    /// Compilation category.
    Compilation,

    /// State creation category.
    StateCreation,

    /// State setting category.
    StateSetting,

    /// State getting category.
    StateGetting,

    /// Resource manipulation category.
    ResourceManipulation,

    /// Execution category.
    Execution,

    /// Shader category.
    Shader,
}
