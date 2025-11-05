use crate::{DWORD, HANDLE, WPARAM};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetRawInputDeviceInfo, RAWHID, RAWINPUT, RIM_TYPEHID, RIM_TYPEKEYBOARD, RIM_TYPEMOUSE, WM_INPUT,
};

/// Contains the header information that is part of the raw input data.
///
/// # Remarks
/// To get more information on the device, use `device` in a call to [`GetRawInputDeviceInfo`].
/// `device` can be zero if an input is received from a precision touchpad.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RAWINPUTHEADER {
    /// The type of raw input. It can be one of the following values:
    ///  * [`RIM_TYPEMOUSE`] - Raw input comes from the mouse.
    ///  * [`RIM_TYPEKEYBOARD`] - Raw input comes from the keyboard.
    ///  * [`RIM_TYPEHID`] - Raw input comes from some device that is not a keyboard or a mouse.
    pub r#type: DWORD,

    /// The size, in bytes, of the entire input packet of data. This includes [`RAWINPUT`] plus
    /// possible extra input reports in the [`RAWHID`] variable length array.
    pub size: DWORD,

    /// A handle to the device generating the raw input data.
    pub device: HANDLE,

    /// The value passed in the `w_param` parameter of the [`WM_INPUT`] message.
    pub w_param: WPARAM,
}

impl Default for RAWINPUTHEADER {
    fn default() -> Self {
        RAWINPUTHEADER {
            r#type: 0,
            size: 0,
            device: null_mut(),
            w_param: 0,
        }
    }
}
