use crate::{d3d11::D3D11_OMAC, GUID, HANDLE, HRESULT, UINT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_INPUT;

/// Contains a response from the [`ID3D11VideoContext::query_authenticated_channel`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    /// A [`D3D11_OMAC`] structure that contains a Message Authentication Code (MAC) of the data.
    /// The driver uses AESbased one-key CBC MAC (OMAC) to calculate this value for the block of
    /// data that appears after this structure member.
    pub omac: D3D11_OMAC,

    /// A [`GUID`] that specifies the query. For a list of possible values, see
    /// [`D3D11_AUTHENTICATED_QUERY_INPUT`]
    pub query_type: GUID,

    /// A handle to the authenticated channel. To get the handle, call the
    /// [`ID3D11AuthenticatedChannel::get_channel_handle`] method.
    pub channel: HANDLE,

    /// The query sequence number.
    pub sequence_number: UINT,

    /// The result code for the query.
    pub return_code: HRESULT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_OUTPUT {
            omac: D3D11_OMAC::default(),
            query_type: GUID::default(),
            channel: null_mut(),
            sequence_number: 0,
            return_code: 0,
        }
    }
}
