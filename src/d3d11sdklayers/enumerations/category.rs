// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11sdklayers::ID3D11InfoQueue;

/// Categories of debug messages. This will identify the category of a message when retrieving a
/// message with [`ID3D11InfoQueue::get_message`] and when adding a message with
/// [`ID3D11InfoQueue::add_message`]. When creating an info queue filter, these values can be used
/// to allow or deny any categories of messages to pass through the storage and retrieval filters.
///
/// # Remarks
/// This is part of the Information Queue feature. See [`ID3D11InfoQueue`] Interface.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_MESSAGE_CATEGORY {
    /// User defined message. See [`ID3D11InfoQueue::add_message`].
    ApplicationDefined = 0,

    #[allow(missing_docs)]
    Miscellaneous,

    #[allow(missing_docs)]
    Initialization,

    #[allow(missing_docs)]
    Cleanup,

    #[allow(missing_docs)]
    Compilation,

    #[allow(missing_docs)]
    StateCreation,

    #[allow(missing_docs)]
    StateSetting,

    #[allow(missing_docs)]
    StateGetting,

    #[allow(missing_docs)]
    ResourceManipulation,

    #[allow(missing_docs)]
    Execution,

    #[allow(missing_docs)]
    Shader,
}
