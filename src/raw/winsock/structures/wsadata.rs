use crate::raw::Word;
use std::ffi::{c_char, c_ushort};

pub type LPWSADATA = *mut WSADATA;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::WSAStartup;

/// # WSADATA structure (winsock.h)
///
/// The [`WSADATA`] structure contains information about the Windows Sockets
/// implementation.
///
/// ## Members
/// `version`
///
/// The version of the Windows Sockets specification that the Ws2_32.dll
/// expects the caller to use. The high-order byte specifies the minor version
/// number; the low-order byte specifies the major version number.
///
///
/// `high_version`
///
/// The highest version of the Windows Sockets specification that the
/// Ws2_32.dll can support. The high-order byte specifies the minor version
/// number; the low-order byte specifies the major version number.
///
/// This is the same value as the `version` member when the version requested
/// in the `version_required` parameter passed to the [`WSAStartup`] function
/// is the highest version of the Windows Sockets specification that the
/// Ws2_32.dll can support.
///
/// `max_sockets`
///
/// The maximum number of sockets that may be opened. This member should be
/// ignored for Windows Sockets version 2 and later.
///
/// The `max_sockets` member is retained for compatibility with Windows Sockets
/// specification 1.1, but should not be used when developing new applications.
/// No single value can be appropriate for all underlying service providers.
/// The architecture of Windows Sockets changed in version 2 to support
/// multiple providers, and the [`WSADATA`] structure no longer applies to a
/// single vendor's stack.
///
/// `max_udp_dg`
///
/// The maximum datagram message size. This member is ignored for Windows
/// Sockets version 2 and later.
///
/// The `max_udp_dg` member is retained for compatibility with Windows Sockets
/// specification 1.1, but should not be used when developing new applications.
/// The architecture of Windows Sockets changed in version 2 to support
/// multiple providers, and the [`WSADATA`] structure no longer applies to a
/// single vendor's stack. For the actual maximum message size specific to a
/// particular Windows Sockets service provider and socket type, applications
/// should use getsockopt to retrieve the value of option SO_MAX_MSG_SIZE after
/// a socket has been created.
///
/// `vendor_info`
///
/// A pointer to vendor-specific information. This member should be ignored for
/// Windows Sockets version 2 and later.
///
/// The `vendor_info` member is retained for compatibility with Windows Sockets
/// specification 1.1. The architecture of Windows Sockets changed in version 2
/// to support multiple providers, and the [`WSADATA`] structure no longer
/// applies to a single vendor's stack. Applications needing to access
/// vendor-specific configuration information should use getsockopt to retrieve
/// the value of option PVD_CONFIG for vendor-specific information.
///
/// `description`
///
/// A NULL-terminated ASCII string into which the Ws2_32.dll copies a
/// description of the Windows Sockets implementation. The text (up to 256
/// characters in length) can contain any characters except control and
/// formatting characters. The most likely use that an application would have
/// for this member is to display it (possibly truncated) in a status message.
///
/// `system_status`
///
/// A NULL-terminated ASCII string into which the Ws2_32.dll copies relevant
/// status or configuration information. The Ws2_32.dll should use this
/// parameter only if the information might be useful to the user or support
/// staff. This member should not be considered as an extension of the
/// `description` parameter.
///
/// ## Remarks
/// The [`WSAStartup`] function initiates the use of the Windows Sockets DLL by
/// a process. The [`WSAStartup`] function returns a pointer to the [`WSADATA`]
/// structure in the `wsa_data` parameter.
///
/// The current version of the Windows Sockets specification returned in the
/// `high_version` member of the [`WSADATA`] structure is version 2.2 encoded
/// with the major version number in the low-byte and the minor version number
/// in the high-byte. This version of the current Winsock DLL, Ws2_32.dll,
/// supports applications that request any of the following versions of the
/// Windows Sockets specification:
///  - 1.0
///  - 1.1
///  - 2.0
///  - 2.1
///  - 2.2
///
/// Depending on the version requested by the application, one of the above
/// version numbers is the value encoded as the major version number in the
/// low-byte and the minor version number in the high-byte that is returned
/// in the `version` member of the [`WSADATA`] structure.
#[repr(C)]
#[derive(Clone)]
pub struct WSADATA {
    pub version: Word,
    pub high_version: Word,
    pub max_sockets: c_ushort,
    pub max_udp_dg: c_ushort,
    pub vendor_info: *mut c_char,
    pub description: [c_char; WSADESCRIPTION_LEN + 1],
    pub system_status: [c_char; WSASYS_STATUS_LEN + 1],
}

pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;

impl WSADATA {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
