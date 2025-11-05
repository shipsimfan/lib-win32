use crate::{RAWHID, RAWINPUTHEADER, RAWKEYBOARD, RAWMOUSE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetRawInputBuffer, GetRawInputData, GetRawInputDeviceInfo, RegisterRawInputDevices, WM_INPUT,
};

/// Contains the raw input from a device.
///
/// # Remarks
/// The handle to this structure is passed in the `l_param` parameter of [`WM_INPUT`].
///
/// To get detailed information -- such as the header and the content of the raw input -- call
/// [`GetRawInputData`].
///
/// To read the [`RAWINPUT`] in the message loop as a buffered read, call [`GetRawInputBuffer`].
///
/// To get device specific information, call [`GetRawInputDeviceInfo`] with the `device` from
/// [`RAWINPUTHEADER`].
///
/// Raw input is available only when the application calls [`RegisterRawInputDevices`] with valid
/// device specifications.
#[repr(C)]
#[derive(Clone)]
pub struct RAWINPUT {
    /// The raw input data.
    pub header: RAWINPUTHEADER,

    #[allow(missing_docs)]
    pub data: RAWINPUTUNION,
}

impl Default for RAWINPUT {
    fn default() -> Self {
        RAWINPUT {
            header: RAWINPUTHEADER::default(),
            data: RAWINPUTUNION::default(),
        }
    }
}

/// Union of input device types for [`RAWINPUT`]
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAWINPUTUNION {
    /// If the data comes from a mouse, this is the raw input data.
    pub mouse: RAWMOUSE,

    /// If the data comes from a keyboard, this is the raw input data.
    pub keyboard: RAWKEYBOARD,

    /// If the data comes from an HID, this is the raw input data.
    pub hid: RAWHID,
}

impl Default for RAWINPUTUNION {
    fn default() -> Self {
        RAWINPUTUNION {
            mouse: RAWMOUSE::default(),
        }
    }
}
