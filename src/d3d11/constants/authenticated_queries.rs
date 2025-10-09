use crate::GUID;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT, D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT,
    D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT,
    D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT,
};

/// Returns the current protection level for the device.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: GUID = GUID {
    data1: 0xA84EB584,
    data2: 0xC495,
    data3: 0x48AA,
    data4: [0xB9, 0x4D, 0x8B, 0xD2, 0xD6, 0xFB, 0xCE, 0x5],
};

/// Returns the type of authenticated channel.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: GUID = GUID {
    data1: 0xBC1B18A5,
    data2: 0xB1FB,
    data3: 0x42AB,
    data4: [0xBD, 0x94, 0xB5, 0x82, 0x8B, 0x4B, 0xF7, 0xBE],
};

/// Returns a handle to the device that is associated with this authenticated channel.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: GUID = GUID {
    data1: 0xEC1C539D,
    data2: 0x8CFF,
    data3: 0x4E2A,
    data4: [0xBC, 0xC4, 0xF5, 0x69, 0x2F, 0x99, 0xF4, 0x80],
};

/// Returns handles to the cryptographic session and Direct3D device that are associated with a
/// specified decoder device.
///
/// Input data structure: [`D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT`]
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: GUID = GUID {
    data1: 0x2634499E,
    data2: 0xD018,
    data3: 0x4D74,
    data4: [0xAC, 0x17, 0x7F, 0x72, 0x40, 0x59, 0x52, 0x8D],
};

/// Returns the number of processes that are allowed to open shared resources with restricted
/// access.
///
/// Output data structure:
/// [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: GUID = GUID {
    data1: 0xDB207B3,
    data2: 0x9450,
    data3: 0x46A6,
    data4: [0x82, 0xDE, 0x1B, 0x96, 0xD4, 0x4F, 0x9C, 0xF2],
};

/// Returns information about a process that is allowed to open shared resources with restricted
/// access.
///
/// Input data structure: [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT`]
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: GUID = GUID {
    data1: 0x649BBADB,
    data2: 0xF0F4,
    data3: 0x4639,
    data4: [0xA1, 0x5B, 0x24, 0x39, 0x3F, 0xC3, 0xAB, 0xAC],
};

/// Returns the number of protected shared resources that can be opened by any process with no
/// restrictions.
///
/// Output data structure:
/// [`D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: GUID = GUID {
    data1: 0x12F0BD6,
    data2: 0xE662,
    data3: 0x4474,
    data4: [0xBE, 0xFD, 0xAA, 0x53, 0xE5, 0x14, 0x3C, 0x6D],
};

/// Returns the number of output identifiers that are associated with a specified cryptographic
/// session and Direct3D device.
///
/// Input data structure: [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT`]
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: GUID = GUID {
    data1: 0x2C042B5E,
    data2: 0x8C07,
    data3: 0x46D5,
    data4: [0xAA, 0xBE, 0x8F, 0x75, 0xCB, 0xAD, 0x4C, 0x31],
};

/// Returns one of the output identifiers that is associated with a specified cryptographic session
/// and Direct3D device.
///
/// Input data structure: [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT`]
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: GUID = GUID {
    data1: 0x839DDCA3,
    data2: 0x9B4E,
    data3: 0x41E4,
    data4: [0xB0, 0x53, 0x89, 0x2B, 0xD2, 0xA1, 0x1E, 0xE7],
};

/// Returns the type of I/O bus that is used to send data to the GPU.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: GUID = GUID {
    data1: 0x6214D9D2,
    data2: 0x432C,
    data3: 0x4ABB,
    data4: [0x9F, 0xCE, 0x21, 0x6E, 0xEA, 0x26, 0x9E, 0x3B],
};

/// Returns the number of encryption types that can be used to encrypt content before it becomes
/// accessible to the CPU or bus.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: GUID = GUID {
    data1: 0xB30F7066,
    data2: 0x203C,
    data3: 0x4B07,
    data4: [0x93, 0xFC, 0xCE, 0xAA, 0xFD, 0x61, 0x24, 0x1E],
};

/// Returns one of the encryption types that can be used to encrypt content before it becomes
/// accessible to the CPU or bus.
///
/// Input data structure: [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT`]
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: GUID = GUID {
    data1: 0xF83A5958,
    data2: 0xE986,
    data3: 0x4BDA,
    data4: [0xBE, 0xB0, 0x41, 0x1F, 0x6A, 0x7A, 0x1, 0xB7],
};

/// Returns the encryption type that is applied before content becomes accessible to the CPU or
/// bus.
///
/// Output data structure: [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT`]
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: GUID = GUID {
    data1: 0xEC1791C7,
    data2: 0xDAD3,
    data3: 0x4F15,
    data4: [0x9E, 0xC3, 0xFA, 0xA9, 0x3D, 0x60, 0xD4, 0xF0],
};
