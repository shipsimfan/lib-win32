use crate::{DWORD, RID_DEVICE_INFO_HID, RID_DEVICE_INFO_KEYBOARD, RID_DEVICE_INFO_MOUSE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{RIM_TYPEHID, RIM_TYPEKEYBOARD, RIM_TYPEMOUSE};

/// Defines the raw input data coming from any device.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct RID_DEVICE_INFO {
    /// The size, in bytes, of the [`RID_DEVICE_INFO`] structure.
    pub size: DWORD,

    /// The type of raw input data. This member can be one of the following values:
    ///  * [`RIM_TYPEMOUSE`] - Data comes from a mouse.
    ///  * [`RIM_TYPEKEYBOARD`] - Data comes from a keyboard.
    ///  * [`RIM_TYPEHID`] - Data comes from an HID that is not a keyboard or a mouse.
    pub r#type: DWORD,

    #[allow(missing_docs)]
    pub u: RID_DEVICE_INFO_UNION,
}

impl Default for RID_DEVICE_INFO {
    fn default() -> Self {
        RID_DEVICE_INFO {
            size: 0,
            r#type: 0,
            u: RID_DEVICE_INFO_UNION::default(),
        }
    }
}

/// Union for [`RID_DEVICE_INFO`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union RID_DEVICE_INFO_UNION {
    /// If `r#type` is [`RIM_TYPEMOUSE`], this is the [`RID_DEVICE_INFO_MOUSE`] structure that
    /// defines the mouse.
    pub mouse: RID_DEVICE_INFO_MOUSE,

    /// If `r#type` is [`RIM_TYPEKEYBOARD`], this is the [`RID_DEVICE_INFO_KEYBOARD`] structure
    /// that defines the keyboard.
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,

    /// If `r#type` is [`RIM_TYPEHID`], this is the [`RID_DEVICE_INFO_HID`] structure that defines
    /// the HID device.
    pub hid: RID_DEVICE_INFO_HID,
}

impl Default for RID_DEVICE_INFO_UNION {
    fn default() -> Self {
        RID_DEVICE_INFO_UNION {
            mouse: RID_DEVICE_INFO_MOUSE::default(),
        }
    }
}
