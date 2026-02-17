use crate::{BOOL, PCRAWINPUTDEVICE, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, DWORD, FALSE, RAWINPUTDEVICE, RIDEV_DEVNOTIFY, TRUE, WM_INPUT,
    WM_INPUT_DEVICE_CHANGE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Registers the devices that supply the raw input data.
    ///
    /// # Parameters
    ///  * `raw_input_devices` - An array of [`RAWINPUTDEVICE`] structures that represent the
    ///                          devices that supply the raw input. Pointer should be aligned on a
    ///                          [`DWORD`] (32-bit) boundary.
    ///  * `num_devices` - The number of [`RAWINPUTDEVICE`] structures pointed to by
    ///                    `raw_input_devices`.
    ///  * `size` - The size, in bytes, of a [`RAWINPUTDEVICE`] structure.
    ///
    /// # Return Value
    /// [`TRUE`] if the function succeeds; otherwise, [`FALSE`]. If the function fails, call
    /// [`GetLastError`] for more information.
    ///
    /// # Remarks
    /// To receive [`WM_INPUT`] messages, an application must first register the raw input devices
    /// using [`RegisterRawInputDevices`]. By default, an application does not receive raw input.
    ///
    /// To receive [`WM_INPUT_DEVICE_CHANGE`] messages, an application must specify the
    /// [`RIDEV_DEVNOTIFY`] flag for each device class that is specified by the `usage_page` and
    /// `usage` fields of the [`RAWINPUTDEVICE`] structure . By default, an application does not
    /// receive [`WM_INPUT_DEVICE_CHANGE`] notifications for raw input device arrival and removal.
    ///
    /// If a [`RAWINPUTDEVICE`] structure has the [`RIDEV_REMOVE`] flag set and the `wnd_target`
    /// parameter is not set to [`null_mut`], then parameter validation will fail.
    ///
    /// Only one window per raw input device class may be registered to receive raw input within a
    /// process (the window passed in the last call to [`RegisterRawInputDevices`]). Because of
    /// this, [`RegisterRawInputDevices`] should not be used from a library, as it may interfere
    /// with any raw input processing logic already present in applications that load it.
    pub fn RegisterRawInputDevices(
        raw_input_devices: PCRAWINPUTDEVICE,
        num_devices: UINT,
        size: UINT,
    ) -> BOOL;
}
