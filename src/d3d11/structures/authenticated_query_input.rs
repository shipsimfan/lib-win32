use crate::{GUID, HANDLE, UINT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES, D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE,
    D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION,
    D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE,
    D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE,
    D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID,
    D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT,
    D3D11_AUTHENTICATED_QUERY_OUTPUT_ID, D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT,
    D3D11_AUTHENTICATED_QUERY_PROTECTION,
    D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS,
    D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT,
    D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT,
};

/// Contains input data for the [`ID3D11VideoContext::query_authenticated_channel`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    /// A [`GUID`] that specifies the query. The following [`GUID`]s are defined:
    ///  * [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_PROTECTION`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT`]
    ///  * [`D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT`]
    pub query_type: GUID,

    /// A handle to the authenticated channel. To get the handle, call the
    /// [`ID3D11AuthenticatedChannel::get_channel_handle`] method.
    pub channel: HANDLE,

    /// The query sequence number. At the start of the session, generate a cryptographically secure
    /// 32-bit random number to use as the starting sequence number. For each query, increment the
    /// sequence number by 1.
    pub sequence_number: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_INPUT {
            query_type: GUID::default(),
            channel: null_mut(),
            sequence_number: 0,
        }
    }
}
