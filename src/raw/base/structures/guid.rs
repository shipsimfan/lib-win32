use std::ffi::{c_char, c_ulong, c_ushort};

/// # GUID structure (guiddef.h)
///
/// A GUID identifies an object such as a COM interfaces, or a COM class
/// object, or a manager entry-point vector (EPV). A GUID is a 128-bit value
/// consisting of one group of 8 hexadecimal digits, followed by three groups
/// of 4 hexadecimal digits each, followed by one group of 12 hexadecimal
/// digits. The following example GUID shows the groupings of hexadecimal
/// digits in a GUID: 6B29FC40-CA47-1067-B31D-00DD010662DA.
///
/// The GUID structure stores a GUID.
///
/// ## Members
/// `0`\
/// Specifies the first 8 hexadecimal digits of the GUID.
///
/// `1`\
/// Specifies the first group of 4 hexadecimal digits.
///
/// `2`\
/// Specifies the second group of 4 hexadecimal digits.
///
/// `3`\
/// Array of 8 bytes. The first 2 bytes contain the third group of 4
/// hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal
/// digits.
///
/// ## Remarks
/// GUIDs are the Microsoft implementation of the distributed computing
/// environment (DCE) universally unique identifier (UUID). The RPC run-time
/// libraries use UUIDs to check for compatibility between clients and servers
/// and to select among multiple implementations of an interface. The Windows
/// access-control functions use GUIDs to identify the type of object that an
/// object-specific ACE in an access-control list (ACL) protects.
#[repr(C)]
#[derive(Clone)]
pub struct GUID(pub c_ulong, pub c_ushort, pub c_ushort, pub [c_char; 8]);
