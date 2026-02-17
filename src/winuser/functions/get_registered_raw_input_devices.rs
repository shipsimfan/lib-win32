use crate::{PRAWINPUTDEVICE, PUINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, RegisterRawInputDevices, DWORD, ERROR_INSUFFICIENT_BUFFER, RAWINPUTDEVICE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves the information about the raw input devices for the current application.
    ///
    /// # Parameters
    ///  * `raw_input_devices` - An array of [`RAWINPUTDEVICE`] structures for the application.
    ///                          Pointer should be aligned on a [`DWORD`] (32-bit) boundary.
    ///  * `num_devices` - The number of [`RAWINPUTDEVICE`] structures in `*raw_input_devices`.
    ///  * `size` - The size, in bytes, of a [`RAWINPUTDEVICE`] structure.
    ///
    /// # Return Value
    /// If successful, the function returns a non-negative number that is the number of
    /// [`RAWINPUTDEVICE`] structures written to the buffer.
    ///
    /// If the `raw_input_devices` buffer is too small or [`null_mut`], the function sets the last
    /// error as [`ERROR_INSUFFICIENT_BUFFER`], returns -1, and sets `num_devices` to the required
    /// number of devices. If the function fails for any other reason, it returns -1. For more
    /// details, call [`GetLastError`].
    ///
    /// # Remarks
    /// To receive raw input from a device, an application must register it by using
    /// [`RegisterRawInputDevices`].
    pub fn GetRegisteredRawInputDevices(
        raw_input_devices: PRAWINPUTDEVICE,
        num_devices: PUINT,
        size: UINT,
    ) -> UINT;
}
