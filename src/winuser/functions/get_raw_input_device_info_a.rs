use crate::{HANDLE, LPVOID, PUINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetLastError, GetRawInputDeviceInfo, GetRawInputDeviceList, ReadFile, WriteFile,
    DWORD, PHIDP_PREPARSED_DATA, RAWINPUTHEADER, RIDI_DEVICEINFO, RIDI_DEVICENAME,
    RIDI_PREPARSEDDATA, RID_DEVICE_INFO,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves information about the raw input device.
    ///
    /// # Parameters
    ///  * `device` - A handle to the raw input device. This comes from the `device` member of
    ///               [`RAWINPUTHEADER`] or from [`GetRawInputDeviceList`].
    ///  * `command` - Specifies what data will be returned in `data`. This parameter can be one of
    ///                the following values:
    ///    * [`RIDI_PREPARSEDDATA`] -  `data` is a [`PHIDP_PREPARSED_DATA`] pointer to a buffer for
    ///                                a top-level collection's preparsed data.
    ///    * [`RIDI_DEVICENAME`] - `data` points to a string that contains the device interface
    ///                            name. If this device is opened with Shared Access Mode then you
    ///                            can call [`CreateFile`] with this name to open a HID collection
    ///                            and use returned handle for calling [`ReadFile`] to read input
    ///                            reports and [`WriteFile`] to send output reports. For this
    ///                            `command` only, the value in `size` is the character count (not
    ///                            the byte count).
    ///    * [`RIDI_DEVICEINFO`] -  `data` points to an [`RID_DEVICE_INFO`] structure.
    ///  * `data` - A pointer to a buffer that contains the information specified by `command`.
    ///             Pointer should be aligned on a [`DWORD`] (32-bit) boundary. If `command` is
    ///             [`RIDI_DEVICEINFO`], set the `size` member of [`RID_DEVICE_INFO`] to
    ///             `std::mem::size_of::<RID_DEVICE_INFO>()` before calling
    ///             [`GetRawInputDeviceInfo`].
    ///  * `size` - The size, in bytes, of the data in `data`.
    ///
    /// # Return Value
    /// If successful, this function returns a non-negative number indicating the number of bytes
    /// copied to `data`.
    ///
    /// If `data` is not large enough for the data, the function returns -1. If `data` is
    /// [`null_mut`], the function returns a value of zero. In both of these cases, `size` is set
    /// to the minimum size required for the `data` buffer.
    ///
    /// Call [`GetLastError`] to identify any other errors.
    pub fn GetRawInputDeviceInfoA(device: HANDLE, command: UINT, data: LPVOID, size: PUINT)
        -> UINT;
}
