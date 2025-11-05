use crate::{BOOL, DWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, HORIZONTAL_WHEEL_PRESENT, MOUSE_HID_HARDWARE, TRUE, WHEELMOUSE_HID_HARDWARE};

/// Defines the raw input data coming from the specified mouse.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct RID_DEVICE_INFO_MOUSE {
    /// The bitfield of the mouse device identification properties:
    ///  * [`MOUSE_HID_HARDWARE`] - HID mouse
    ///  * [`WHEELMOUSE_HID_HARDWARE`] - HID wheel mouse
    ///  * [`HORIZONTAL_WHEEL_PRESENT`] - Mouse with horizontal wheel
    pub id: DWORD,

    /// The number of buttons for the mouse.
    pub number_of_buttons: DWORD,

    /// The number of data points per second. This information may not be applicable for every
    /// mouse device.
    pub sample_rate: DWORD,

    /// [`TRUE`] if the mouse has a wheel for horizontal scrolling; otherwise, [`FALSE`].
    pub has_horizontal_wheel: BOOL,
}

impl Default for RID_DEVICE_INFO_MOUSE {
    fn default() -> Self {
        RID_DEVICE_INFO_MOUSE {
            id: 0,
            number_of_buttons: 0,
            sample_rate: 0,
            has_horizontal_wheel: 0,
        }
    }
}
