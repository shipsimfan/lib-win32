use crate::{BYTE, DWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::WM_INPUT;

/// Describes the format of the raw input from a Human Interface Device (HID).
///
/// # Remarks
/// Each [`WM_INPUT`] can indicate several inputs, but all of the inputs come from the same HID.
/// The size of the `raw_data` array is `size_hid * count`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RAWHID {
    /// The size, in bytes, of each HID input in `raw_data`.
    pub size_hid: DWORD,

    /// The number of HID inputs in `raw_data`.
    pub count: DWORD,

    /// The raw input data, as an array of bytes.
    pub raw_data: [BYTE; 1],
}

impl Default for RAWHID {
    fn default() -> Self {
        RAWHID {
            size_hid: 0,
            count: 0,
            raw_data: [0; 1],
        }
    }
}
