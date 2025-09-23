use crate::{d3d11::D3D11_OMAC, GUID, HANDLE, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT,
    D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION,
    D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT,
    D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE,
    D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE, D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT,
    D3D11_AUTHENTICATED_CONFIGURE_PROTECTION, D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT,
    D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE,
    D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT,
};

/// Contains input data for the [`ID3D11VideoContext::configure_authenticated_channel`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    /// A [`D3D11_OMAC`] structure that contains a Message Authentication Code (MAC) of the data.
    /// The driver uses AES-based one-key CBC MAC (OMAC) to calculate this value for the block of
    /// data that appears after this structure member.
    pub omac: D3D11_OMAC,

    /// A [`GUID`] that specifies the command. The following GUIDs are defined:
    ///  * [`D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION`] -  Associates a cryptographic session
    ///                                                        with a decoder device and a Direct3D
    ///                                                        device. Input data:
    ///                                      [`D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT`]
    ///  * [`D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE`] -  Sets the level of
    ///                                                                    encryption that is
    ///                                                                    performed before
    ///                                                                    protected content
    ///                                                                    becomes accessible to
    ///                                                                    the CPU or bus. Input
    ///                                                                    data:
    ///                               [`D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT`]
    ///  * [`D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE`] -  Initializes the authenticated channel.
    ///                                                    Input data:
    ///                                          [`D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT`]
    ///  * [`D3D11_AUTHENTICATED_CONFIGURE_PROTECTION`] -  Enables or disables protection for the
    ///                                                    device. Input data:
    ///                                          [`D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT`]
    ///  * [`D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE`] -  Enables a process to open a shared
    ///                                                         resource, or disables a process
    ///                                                         from opening shared resources.
    ///                                                         Input data:
    ///                                     [`D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT`]
    pub configure_type: GUID,

    /// A handle to the authenticated channel. To get the handle, call the
    /// [`ID3D11AuthenticatedChannel::get_channel_handle`] method.
    pub channel: HANDLE,

    /// The query sequence number. At the start of the session, generate a cryptographically secure
    /// 32-bit random number to use as the starting sequence number. For each query, increment the
    /// sequence number by 1.
    pub sequence_number: UINT,
}
