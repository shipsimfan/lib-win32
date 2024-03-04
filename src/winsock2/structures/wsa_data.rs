use crate::{
    winsock2::{WSADESCRIPTION_LEN, WSASYS_STATUS_LEN},
    WORD,
};
use std::{
    ffi::{c_char, c_ushort},
    ptr::null_mut,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::WSAStartup;

/// The [`WSADATA`] structure contains information about the Windows Sockets implementation.
#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WSADATA {
    /// The version of the Windows Sockets specification that the Ws2_32.dll expects the caller to
    /// use. The high-order byte specifies the minor version number; the low-order byte specifies
    /// the major version number.
    pub version: WORD,

    /// The highest version of the Windows Sockets specification that the Ws2_32.dll can support.
    /// The high-order byte specifies the minor version number; the low-order byte specifies the
    /// major version number.
    ///
    /// This is the same value as the `version` member when the version requested in the
    /// `version_requested` parameter passed to the [`WSAStartup`] function is the highest version
    /// of the Windows Sockets specification that the Ws2_32.dll can support.
    pub high_version: WORD,

    /// The maximum number of sockets that may be opened. This member should be ignored for Windows
    /// Sockets version 2 and later.
    ///
    /// The `max_sockets` member is retained for compatibility with Windows Sockets specification
    /// 1.1, but should not be used when developing new applications. No single value can be
    /// appropriate for all underlying service providers. The architecture of Windows Sockets
    /// changed in version 2 to support multiple providers, and the [`WSADATA`] structure no longer
    /// applies to a single vendor's stack.
    pub max_sockets: c_ushort,

    /// The maximum datagram message size. This member is ignored for Windows Sockets version 2 and
    /// later.
    ///
    /// The `max_udp_dg` member is retained for compatibility with Windows Sockets specification
    /// 1.1, but should not be used when developing new applications. The architecture of Windows
    /// Sockets changed in version 2 to support multiple providers, and the [`WSADATA`] structure
    /// no longer applies to a single vendor's stack. For the actual maximum message size specific
    /// to a particular Windows Sockets service provider and socket type, applications should use
    /// [`getsockopt`] to retrieve the value of option [`SO_MAX_MSG_SIZE`] after a socket has been
    /// created.
    pub max_udp_dg: c_ushort,

    /// A pointer to vendor-specific information. This member should be ignored for Windows Sockets
    /// version 2 and later.
    ///
    /// The `vendor_info` member is retained for compatibility with Windows Sockets specification
    /// 1.1. The architecture of Windows Sockets changed in version 2 to support multiple
    /// providers, and the [`WSADATA`] structure no longer applies to a single vendor's stack.
    /// Applications needing to access vendor-specific configuration information should use
    /// [`getsockopt`] to retrieve the value of option [`PVD_CONFIG`] for vendor-specific
    /// information.
    pub vendor_info: *mut c_char,

    /// A null-terminated ASCII string into which the Ws2_32.dll copies a description of the
    /// Windows Sockets implementation. The text (up to 256 characters in length) can contain any
    /// characters except control and formatting characters. The most likely use that an
    /// application would have for this member is to display it (possibly truncated) in a status
    /// message.
    pub description: [c_char; WSADESCRIPTION_LEN + 1],

    /// A null-terminated ASCII string into which the Ws2_32.dll copies relevant status or
    /// configuration information. The Ws2_32.dll should use this parameter only if the information
    /// might be useful to the user or support staff. This member should not be considered as an
    /// extension of the `description` parameter.
    pub system_status: [c_char; WSASYS_STATUS_LEN + 1],
}

/// The [`WSADATA`] structure contains information about the Windows Sockets implementation.
#[cfg(target_pointer_width = "32")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WSADATA {
    /// The version of the Windows Sockets specification that the Ws2_32.dll expects the caller to
    /// use. The high-order byte specifies the minor version number; the low-order byte specifies
    /// the major version number.
    pub version: WORD,

    /// The highest version of the Windows Sockets specification that the Ws2_32.dll can support.
    /// The high-order byte specifies the minor version number; the low-order byte specifies the
    /// major version number.
    ///
    /// This is the same value as the `version` member when the version requested in the
    /// `version_requested` parameter passed to the [`WSAStartup`] function is the highest version
    /// of the Windows Sockets specification that the Ws2_32.dll can support.
    pub high_version: WORD,

    /// A null-terminated ASCII string into which the Ws2_32.dll copies a description of the
    /// Windows Sockets implementation. The text (up to 256 characters in length) can contain any
    /// characters except control and formatting characters. The most likely use that an
    /// application would have for this member is to display it (possibly truncated) in a status
    /// message.
    pub description: [c_char; WSADESCRIPTION_LEN + 1],

    /// A null-terminated ASCII string into which the Ws2_32.dll copies relevant status or
    /// configuration information. The Ws2_32.dll should use this parameter only if the information
    /// might be useful to the user or support staff. This member should not be considered as an
    /// extension of the `description` parameter.
    pub system_status: [c_char; WSASYS_STATUS_LEN + 1],

    /// The maximum number of sockets that may be opened. This member should be ignored for Windows
    /// Sockets version 2 and later.
    ///
    /// The `max_sockets` member is retained for compatibility with Windows Sockets specification
    /// 1.1, but should not be used when developing new applications. No single value can be
    /// appropriate for all underlying service providers. The architecture of Windows Sockets
    /// changed in version 2 to support multiple providers, and the [`WSADATA`] structure no longer
    /// applies to a single vendor's stack.
    pub max_sockets: c_ushort,

    /// The maximum datagram message size. This member is ignored for Windows Sockets version 2 and
    /// later.
    ///
    /// The `max_udp_dg` member is retained for compatibility with Windows Sockets specification
    /// 1.1, but should not be used when developing new applications. The architecture of Windows
    /// Sockets changed in version 2 to support multiple providers, and the [`WSADATA`] structure
    /// no longer applies to a single vendor's stack. For the actual maximum message size specific
    /// to a particular Windows Sockets service provider and socket type, applications should use
    /// [`getsockopt`] to retrieve the value of option [`SO_MAX_MSG_SIZE`] after a socket has been
    /// created.
    pub max_udp_dg: c_ushort,

    /// A pointer to vendor-specific information. This member should be ignored for Windows Sockets
    /// version 2 and later.
    ///
    /// The `vendor_info` member is retained for compatibility with Windows Sockets specification
    /// 1.1. The architecture of Windows Sockets changed in version 2 to support multiple
    /// providers, and the [`WSADATA`] structure no longer applies to a single vendor's stack.
    /// Applications needing to access vendor-specific configuration information should use
    /// [`getsockopt`] to retrieve the value of option [`PVD_CONFIG`] for vendor-specific
    /// information.
    pub vendor_info: *mut c_char,
}

impl Default for WSADATA {
    fn default() -> Self {
        WSADATA {
            version: 0,
            high_version: 0,
            max_sockets: 0,
            max_udp_dg: 0,
            vendor_info: null_mut(),
            description: [0; WSADESCRIPTION_LEN + 1],
            system_status: [0; WSASYS_STATUS_LEN + 1],
        }
    }
}
