use crate::{PRAWINPUTDEVICELIST, PUINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GetRawInputDeviceInfo, DWORD, ERROR_INSUFFICIENT_BUFFER, RAWINPUTDEVICELIST,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Enumerates the raw input devices attached to the system.
    ///
    /// # Parameters
    ///  * `raw_input_device_list` - An array of [`RAWINPUTDEVICELIST`] structures for the devices
    ///                              attached to the system. Pointer should be aligned on a
    ///                              [`DWORD`] (32-bit) boundary. If [`null_mut`], the number of
    ///                              devices are returned in `*num_devices`.
    ///  * `num_devices` - If `raw_input_device_list` is [`null_mut`], the function populates this
    ///                    variable with the number of devices attached to the system; otherwise,
    ///                    this variable specifies the number of [`RAWINPUTDEVICELIST`] structures
    ///                    that can be contained in the buffer to which `raw_input_device_list`
    ///                    points. If this value is less than the number of devices attached to the
    ///                    system, the function returns the actual number of devices in this
    ///                    variable and fails with [`ERROR_INSUFFICIENT_BUFFER`]. If this value is
    ///                    greater than or equal to the number of devices attached to the system,
    ///                    then the value is unchanged, and the number of devices is reported as
    ///                    the return value.
    ///  * `size` - The size of a [`RAWINPUTDEVICELIST`] structure, in bytes.
    ///
    /// # Return Value
    /// If the function is successful, the return value is the number of devices stored in the
    /// buffer pointed to by `raw_input_device_list`.
    ///
    /// On any other error, the function returns `-1 as UINT` and [`GetLastError`] returns the
    /// error indication.
    ///
    /// # Remarks
    /// The devices returned from this function are the mouse, the keyboard, and other Human
    /// Interface Device (HID) devices.
    ///
    /// To get more detailed information about the attached devices, call [`GetRawInputDeviceInfo`]
    /// using the `device` from [`RAWINPUTDEVICELIST`].
    ///
    /// Input devices accessed through Remote Desktop Protocal (RDP) do not appear in the raw input
    /// device list.
    pub fn GetRawInputDeviceList(
        raw_input_device_list: PRAWINPUTDEVICELIST,
        num_devices: PUINT,
        size: UINT,
    ) -> UINT;
}
