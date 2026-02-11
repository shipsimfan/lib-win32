use crate::{CHAR, DWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    EnumDisplayDevices, DISPLAY_DEVICE, DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_MIRRORING_DRIVER,
    DISPLAY_DEVICE_MODESPRUNED, DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICE_REMOVABLE,
    DISPLAY_DEVICE_VGA_COMPATIBLE,
};
#[allow(unused_imports)]
use std::ptr::null;

/// The [`DISPLAY_DEVICE`] structure receives information about the display device specified by the
/// `dev_num` parameter of the [`EnumDisplayDevices`] function.
///
/// # Remarks
/// The four string members are set based on the parameters passed to [`EnumDisplayDevices`]. If
/// the `device` param is [`null`], then [`DISPLAY_DEVICE`] is filled in with information about the
/// display adapter(s). If it is a valid device name, then it is filled in with information about
/// the monitor(s) for that device.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAY_DEVICEA {
    /// Size, in bytes, of the [`DISPLAY_DEVICE`] structure. This must be initialized prior to
    /// calling [`EnumDisplayDevices`].
    pub cb: DWORD,

    /// An array of characters identifying the device name. This is either the adapter device or
    /// the monitor device.
    pub device_name: [CHAR; 32],

    /// An array of characters containing the device context string. This is either a description
    /// of the display adapter or of the display monitor.
    pub device_string: [CHAR; 128],

    /// Device state flags. It can be any reasonable combination of the following:
    ///  * [`DISPLAY_DEVICE_ACTIVE`] - [`DISPLAY_DEVICE_ACTIVE`] specifies whether a monitor is
    ///                                presented as being "on" by the respective GDI view. Windows
    ///                                Vista: [`EnumDisplayDevices`] will only enumerate monitors
    ///                                that can be presented as being "on."
    ///  * [`DISPLAY_DEVICE_MIRRORING_DRIVER`] - Represents a pseudo device used to mirror
    ///                                          application drawing for remoting or other
    ///                                          purposes. An invisible pseudo monitor is
    ///                                          associated with this device. For example,
    ///                                          NetMeeting uses it. Note that [`GetSystemMetrics`]
    ///                                          ([`SM_MONITORS`]) only accounts for visible
    ///                                          display monitors.
    ///  * [`DISPLAY_DEVICE_MODESPRUNED`] - The device has more display modes than its output
    ///                                     devices support.
    ///  * [`DISPLAY_DEVICE_PRIMARY_DEVICE`] - The primary desktop is on the device. For a system
    ///                                        with a single display card, this is always set. For
    ///                                        a system with multiple display cards, only one
    ///                                        device can have this set.
    ///  * [`DISPLAY_DEVICE_REMOVABLE`] - The device is removable; it cannot be the primary
    ///                                   display.
    ///  * [`DISPLAY_DEVICE_VGA_COMPATIBLE`] - The device is VGA compatible.
    pub state_flags: DWORD,

    /// Not used.
    pub device_id: [CHAR; 128],

    /// Reserved.
    pub device_key: [CHAR; 128],
}

impl Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        DISPLAY_DEVICEA {
            cb: std::mem::size_of::<DISPLAY_DEVICEA>() as _,
            device_name: [0; 32],
            device_string: [0; 128],
            state_flags: 0,
            device_id: [0; 128],
            device_key: [0; 128],
        }
    }
}
