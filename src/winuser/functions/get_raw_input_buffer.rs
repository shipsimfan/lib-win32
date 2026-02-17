use crate::{PRAWINPUT, PUINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, DWORD, NEXTRAWINPUTBLOCK, QS_RAWINPUT, RAWINPUT, RAWINPUTHEADER, WM_INPUT,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Performs a buffered read of the raw input messages data found in the calling thread's
    /// message queue.
    ///
    /// # Parameters
    ///  * `data` - A pointer to a buffer of [`RAWINPUT`] structures that contain the raw input
    ///             data. Pointer should be aligned on a [`DWORD`] (32-bit) boundary. If
    ///             [`null_mut`], size of the first raw input message data (minimum required
    ///             buffer), in bytes, is returned in `*size`.
    ///  * `size` - The size, in bytes, of the provided [`RAWINPUT`] buffer.
    ///  * `size_header` - The size, in bytes, of the [`RAWINPUTHEADER`] structure.
    ///
    /// # Return Value
    /// If `data` is [`null_mut`] and the function is successful, the return value is zero. If
    /// `data` is not [`null_mut`] and the function is successful, the return value is the number
    /// of [`RAWINPUT`] structures written to pData.
    ///
    /// If an error occurs, the return value is `(UINT)-1`. Call [`GetLastError`] for the error
    /// code.
    ///
    /// # Remarks
    /// When an application receives raw input, its message queue gets a [`WM_INPUT`] message and
    /// the queue status flag [`QS_RAWINPUT`] is set.
    ///
    /// Using [`GetRawInputBuffer`], the raw input data is read in the array of variable size
    /// [`RAWINPUT`] structures and corresponding [`WM_INPUT`] messages are removed from the
    /// calling thread's message queue. You can call this method several times with buffer that
    /// cannot fit all message's data until all raw input messages have been read.
    ///
    /// The [`NEXTRAWINPUTBLOCK`] macro allows an application to traverse an array of [`RAWINPUT`]
    /// structures.
    ///
    /// If all raw input messages have been successfully read from message queue then
    /// [`QS_RAWINPUT`] flag is cleared from the calling thread's message queue status.
    pub fn GetRawInputBuffer(data: PRAWINPUT, size: PUINT, size_header: UINT) -> UINT;
}
