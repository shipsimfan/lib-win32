use crate::{DWORD, HANDLE};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{RIM_TYPEHID, RIM_TYPEKEYBOARD, RIM_TYPEMOUSE};

/// Contains information about a raw input device.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RAWINPUTDEVICELIST {
    /// A handle to the raw input device.
    pub device: HANDLE,

    /// The type of device. This can be one of the following values:
    ///  * [`RIM_TYPEHID`] - The device is an HID that is not a keyboard and not a mouse.
    ///  * [`RIM_TYPEKEYBOARD`] - The device is a keyboard.
    ///  * [`RIM_TYPEMOUSE`] - The device is a mouse.
    pub r#type: DWORD,
}

impl Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        RAWINPUTDEVICELIST {
            device: null_mut(),
            r#type: 0,
        }
    }
}
