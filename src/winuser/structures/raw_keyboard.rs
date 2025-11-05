use crate::{UINT, ULONG, USHORT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    KEYBOARD_OVERRUN_MAKE_CODE, RI_KEY_BREAK, RI_KEY_E0, RI_KEY_E1, RI_KEY_MAKE, WM_KEYDOWN,
    WM_SYSKEYDOWN,
};

/// Contains information about the state of the keyboard.
///
/// # Remarks
/// [`KEYBOARD_OVERRUN_MAKE_CODE`] is a special `make_code` value sent when an invalid or
/// unrecognizable combination of keys is pressed or the number of keys pressed exceeds the limit
/// for this keyboard.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RAWKEYBOARD {
    /// Specifies the scan code associated with a key press.
    pub make_code: USHORT,

    /// Flags for scan code information. It can be one or more of the following:
    ///  * [`RI_KEY_MAKE`] - The key is down.
    ///  * [`RI_KEY_BREAK`] - The key is up.
    ///  * [`RI_KEY_E0`] - The scan code has the E0 prefix.
    ///  * [`RI_KEY_E1`] - The scan code has the E1 prefix.
    pub flags: USHORT,

    /// Reserved; must be zero.
    pub reserved: USHORT,

    /// The corresponding legacy virtual-key code.
    pub v_key: USHORT,

    /// The corresponding legacy keyboard window message, for example [`WM_KEYDOWN`],
    /// [`WM_SYSKEYDOWN`], and so forth.
    pub message: UINT,

    /// The device-specific additional information for the event.
    pub extra_information: ULONG,
}

impl Default for RAWKEYBOARD {
    fn default() -> Self {
        RAWKEYBOARD {
            make_code: 0,
            flags: 0,
            reserved: 0,
            v_key: 0,
            message: 0,
            extra_information: 0,
        }
    }
}
