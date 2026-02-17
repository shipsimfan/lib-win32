use crate::{HRAWINPUT, LPVOID, PUINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetRawInputBuffer, DWORD, RAWINPUT, RAWINPUTHEADER, RID_HEADER, RID_INPUT, WM_INPUT};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves the raw input from the specified device.
    ///
    /// # Parameters
    ///  * `raw_input` - A handle to the [`RAWINPUT`] structure. This comes from the `l_param` in
    ///                  [`WM_INPUT`].
    ///  * `command` - The command flag. This parameter can be one of the following values:
    ///    * [`RID_HEADER`] - Get the header information from the [`RAWINPUT`] structure.
    ///    * [`RID_INPUT`] - Get the raw data from the [`RAWINPUT`] structure.
    ///  * `data` - A pointer to the data that comes from the [`RAWINPUT`] structure. This depends
    ///             on the value of `command`. Pointer should be aligned on a [`DWORD`] (32-bit)
    ///             boundary. If `data` is [`null_mut`], the required size of the buffer is
    ///             returned in `*size`.
    ///  * `size` - The size, in bytes, of the data in `data`.
    ///  * `size_header` - The size, in bytes, of the [`RAWINPUTHEADER`] structure.
    ///
    /// # Return Value
    /// If `data` is [`null_mut`] and the function is successful, the return value is 0. If `data`
    /// is not [`null_mut`] and the function is successful, the return value is the number of bytes
    /// copied into `data`.
    ///
    /// If there is an error, the return value is `(UINT)-1`.
    ///
    /// # Remarks
    /// [`GetRawInputData`] gets the raw input one [`RAWINPUT`] structure at a time. In contrast,
    /// [`GetRawInputBuffer`] gets an array of [`RAWINPUT`] structures.
    pub fn GetRawInputData(
        raw_input: HRAWINPUT,
        command: UINT,
        data: LPVOID,
        size: PUINT,
        size_header: UINT,
    ) -> UINT;
}
