use std::ffi::{c_uchar, c_ulong, c_ushort};

/// A [`GUID`] identifies an object such as a COM interfaces, or a COM class object, or a manager
/// entry-point vector (EPV). A [`GUID`] is a 128-bit value consisting of one group of 8
/// hexadecimal digits, followed by three groups of 4 hexadecimal digits each, followed by one
/// group of 12 hexadecimal digits. The following example [`GUID`] shows the groupings of
/// hexadecimal digits in a [`GUID`]: 6B29FC40-CA47-1067-B31D-00DD010662DA.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GUID {
    /// Specifies the first 8 hexadecimal digits of the GUID.
    pub data1: c_ulong,

    /// Specifies the first group of 4 hexadecimal digits.
    pub data2: c_ushort,

    /// Specifies the second group of 4 hexadecimal digits.
    pub data3: c_ushort,

    /// Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits. The
    /// remaining 6 bytes contain the final 12 hexadecimal digits.
    pub data4: [c_uchar; 8],
}
